use flexion::Path;

/// Words ending -ризоваться.
pub(crate) fn n55359(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31885);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['в', 'р', 'а', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 39936);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 41732);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 42695);
									match s {
										[] => true,
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
				['е', 'н', 'и', 'л', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 39937);
					match s {
						[] => true,
						['е', 'л', 'о', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 41733);
							match s {
								[] => true,
								['л', 'и', 'м', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 43268);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 43269);
											match s {
												[] => true,
												['д', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
						['н', 'е', 'в', 'н', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 36709);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 39938);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 41734);
							match s {
								[] => true,
								['л', 'у', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['й', 'е', 'в', 'н', 'о', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', 'у', 'а', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
					match s {
						[] => true,
						['н', 'а', 'п', 'с', 'и', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'е', 'м', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['е', 'т', 'а', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['к', 'а', s @ ..] => {
							match s {
								[] => true,
								['б', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'а', 'х', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 43270);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 43582);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', 'а', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ю', 'ь', 'п', 'м', 'о', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', 'у', 'б', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['г', 'е', 'т', 'а', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 39939);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 41735);
							match s {
								[] => true,
								['в', 'е', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', 'р', 'е', 'т', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 39940);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 41736);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 42696);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', 'а', 'р', 'т', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'м', '\u{1}', s @ ..] => {
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
				['е', s @ ..] => {
					super::push(out, 39941);
					match s {
						[] => true,
						['л', 'л', 'е', 'б', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['м', s @ ..] => {
							match s {
								[] => true,
								['а', 'р', 'а', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'е', 'г', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 39942);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 41737);
							match s {
								[] => true,
								['л', 'э', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 43271);
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
				_ => false,
			}
		}
		['у', 'т', 'к', 'у', 'р', 'т', 'с', 'е', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 36710);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 39943);
					match s {
						[] => true,
						['а', 'и', 'н', 'и', 'м', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', 'л', s @ ..] => {
			match s {
				[] => true,
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 39944);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 41738);
							match s {
								[] => true,
								['е', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 41739);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 42697);
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

/// Words ending -тизоваться.
pub(crate) fn n55360(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 36711);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 39945);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 41740);
							match s {
								[] => true,
								['а', 'р', 'г', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 39946);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 41741);
							match s {
								[] => true,
								['б', 'р', 'а', 'к', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 36712);
			match s {
				[] => true,
				['р', 'к', 'н', 'о', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 36713);
			match s {
				[] => true,
				['а', 'д', 'н', 'а', 'т', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'п', 'с', 'а', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -физоваться.
pub(crate) fn n55361(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31886);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 36714);
			match s {
				[] => true,
				['о', 'м', 'а', 'т', 'е', 'м', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хизоваться.
pub(crate) fn n55362(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31887);
	match s {
		[] => true,
		['р', 'а', 'р', 'е', 'и', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word запломбироваться.
pub(crate) fn n89605(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word опломбироваться.
pub(crate) fn n89606(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word распломбироваться.
pub(crate) fn n89607(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word гравироваться.
pub(crate) fn n87121(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42705);
	s.is_empty()
}

/// Words ending -ггравироваться.
pub(crate) fn n87122(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42706);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 43277);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тгравироваться.
pub(crate) fn n87123(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42707);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 43278);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыгравироваться.
pub(crate) fn n87124(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42708);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 43279);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ивироваться.
pub(crate) fn n68584(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36783);
	match s {
		[] => true,
		['с', 'с', 'а', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 39996);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 41769);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['з', 'е', 'д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', 'и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', 'ъ', s @ ..] => {
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 43280);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['у', 'с', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['д', s @ ..] => {
									super::push(out, 43281);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 43583);
											match s {
												[] => true,
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
					super::push(out, 41770);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 42709);
							match s {
								[] => true,
								['т', 'с', 'б', 'у', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 41771);
					match s {
						[] => true,
						['м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 41772);
					match s {
						[] => true,
						['л', 'у', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 43584);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 43744);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t32::n90258(s, out),
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
									super::push(out, 43282);
									match s {
										[] => true,
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
			super::push(out, 39997);
			match s {
				[] => true,
				['р', 'а', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 42710);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 42711);
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

/// Words ending -рвироваться.
pub(crate) fn n68585(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36784);
	match s {
		[] => true,
		['е', s @ ..] => {
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 41773);
					match s {
						[] => true,
						['е', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 41774);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 41775);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42712);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 42713);
							match s {
								[] => true,
								['о', 'к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 43585);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 43586);
											match s {
												[] => true,
												['з', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 43587);
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьвироваться.
pub(crate) fn n68586(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36785);
	match s {
		[] => true,
		['л', 'а', 'в', 'е', s @ ..] => {
			match s {
				[] => true,
				['д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гироваться.
pub(crate) fn n55467(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31945);
	match s {
		[] => true,
		['а', 'р', s @ ..] => {
			match s {
				[] => true,
				['д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'с', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 41776);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 42714);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', 'э', '\u{1}', s @ ..] => {
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
				['л', s @ ..] => {
					super::push(out, 39998);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', 'г', s @ ..] => {
					match s {
						[] => true,
						['а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 41777);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 42715);
									match s {
										[] => true,
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
			super::push(out, 36786);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 39999);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 41778);
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
				['ф', s @ ..] => {
					super::push(out, 40000);
					match s {
						[] => true,
						['е', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ф', 'у', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 36787);
			match s {
				[] => true,
				['е', 'л', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'л', 'о', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', 'л', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 36788);
			match s {
				[] => true,
				['ф', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'т', 'н', 'е', 'ц', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 36789);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['о', 'ф', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', 'м', 'э', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дироваться.
pub(crate) fn n55468(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31946);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 36790);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 40001);
					match s {
						[] => true,
						['и', 'р', 'р', 'а', 'б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 42716);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 43283);
											match s {
												[] => true,
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
									super::push(out, 42717);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 43284);
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
				['л', s @ ..] => {
					super::push(out, 40002);
					match s {
						[] => true,
						['к', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 40003);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 41779);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 42718);
									match s {
										[] => true,
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
							super::push(out, 41780);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 41781);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 42719);
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
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 36791);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 41782);
							match s {
								[] => true,
								['о', 'т', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 41783);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 42720);
									match s {
										[] => true,
										['э', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 36792);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 40004);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 41784);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 40005);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 41785);
							match s {
								[] => true,
								['в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 41786);
							match s {
								[] => true,
								['с', 'н', 'о', 'к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 43285);
											match s {
												[] => true,
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
					super::push(out, 40006);
					match s {
						[] => true,
						['а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 40007);
					match s {
						[] => true,
						['б', 'у', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['к', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 40008);
					match s {
						[] => true,
						['ь', 'л', 'у', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 36793);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 40009);
					match s {
						[] => true,
						['р', 'г', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 36794);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 40010);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 41787);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 42721);
									match s {
										[] => true,
										['п', 'о', 'р', 'п', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 43588);
													s.is_empty()
												}
												['с', s @ ..] => {
													super::push(out, 43589);
													match s {
														[] => true,
														['а', s @ ..] => super::t32::n90063(s, out),
														_ => false,
													}
												}
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
							super::push(out, 41788);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 42722);
									match s {
										[] => true,
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
							super::push(out, 41789);
							match s {
								[] => true,
								['о', s @ ..] => super::t32::n87153(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => super::t32::n78431(s, out),
				['о', s @ ..] => super::t32::n78432(s, out),
				['у', s @ ..] => super::t32::n78433(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t32::n68599(s, out),
		['р', s @ ..] => super::t32::n68600(s, out),
		['у', s @ ..] => super::t32::n68601(s, out),
		['ь', s @ ..] => super::t32::n68602(s, out),
		['ю', s @ ..] => super::t32::n68603(s, out),
		_ => false,
	}
}

/// Words ending -жироваться.
pub(crate) fn n55469(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 36800);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 36801);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 40021);
					match s {
						[] => true,
						['н', 'а', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 42726);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 43286);
											match s {
												[] => true,
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
					super::push(out, 40022);
					match s {
						[] => true,
						['е', s @ ..] => {
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 42727);
									match s {
										[] => true,
										['д', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 42728);
									match s {
										[] => true,
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
					super::push(out, 40023);
					match s {
						[] => true,
						['у', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 40024);
					match s {
						[] => true,
						['и', 'т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 42729);
									match s {
										[] => true,
										['а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 42730);
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
							super::push(out, 41798);
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
				['с', s @ ..] => {
					super::push(out, 40025);
					match s {
						[] => true,
						['с', 'а', 'м', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 40026);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 41799);
							match s {
								[] => true,
								['а', 'ш', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 41800);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 42731);
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
		['д', s @ ..] => {
			super::push(out, 36802);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['п', 'р', 'а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ф', 'ь', 'л', 'о', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['х', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 42732);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 43287);
											match s {
												[] => true,
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
		['е', 'т', 'о', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 36803);
			match s {
				[] => true,
				['д', 'е', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', 'и', 'д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 36804);
			match s {
				[] => true,
				['а', 'р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 36805);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 40027);
					match s {
						[] => true,
						['ш', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 40028);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 41801);
							match s {
								[] => true,
								['е', 'д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
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

/// Words ending -зироваться.
pub(crate) fn n55470(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31947);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 36806);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 40029);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 41802);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 42733);
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
				['г', s @ ..] => {
					super::push(out, 40030);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 41803);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 42734);
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
				['л', s @ ..] => {
					super::push(out, 40031);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 41804);
							match s {
								[] => true,
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
					super::push(out, 40032);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 41805);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', 'р', 'а', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 40033);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 41806);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 42735);
									match s {
										[] => true,
										['ф', s @ ..] => {
											match s {
												[] => true,
												['а', 'з', '\u{1}', s @ ..] => {
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
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 41807);
							match s {
								[] => true,
								['а', 'т', 'е', 'м', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', 'и', 'г', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 40034);
					match s {
						[] => true,
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
			super::push(out, 36807);
			match s {
				[] => true,
				['т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 41808);
							match s {
								[] => true,
								['и', 'с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 43288);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 43591);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t32::n90064(s, out),
														_ => false,
													}
												}
												_ => false,
											}
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
						['с', 'е', 'н', 'а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t32::n68614(s, out),
		['н', s @ ..] => {
			super::push(out, 36809);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 40045);
					match s {
						[] => true,
						['п', 'с', 'у', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ц', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 43342);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 43637);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t32::n90140(s, out),
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
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 43343);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 43638);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t32::n90141(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 40046);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 41834);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 42785);
									match s {
										[] => true,
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
			super::push(out, 36810);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 40047);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 41835);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 42786);
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
				['н', s @ ..] => {
					super::push(out, 40048);
					match s {
						[] => true,
						['г', 'о', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => super::t32::n68617(s, out),
		_ => false,
	}
}

/// Words ending -иироваться.
pub(crate) fn n55471(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31948);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 36812);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 40050);
					match s {
						[] => true,
						['р', 'б', 'б', 'а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 36813);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 40051);
					match s {
						[] => true,
						['л', 'п', '\u{1}', s @ ..] => {
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
				['п', s @ ..] => {
					super::push(out, 40052);
					match s {
						[] => true,
						['о', 'р', 'п', 'с', 'к', 'э', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', 'а', 'п', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 41836);
							match s {
								[] => true,
								['р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 41837);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 42787);
									match s {
										[] => true,
										['э', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ц', 'и', s @ ..] => {
			super::push(out, 36814);
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

/// Words ending -кироваться.
pub(crate) fn n55472(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31949);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 36815);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 40053);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 41838);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 41839);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 36816);
			match s {
				[] => true,
				['р', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 36817);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 36818);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 40054);
					match s {
						[] => true,
						['л', 'ф', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['м', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'ф', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 36819);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 40055);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 42788);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 42789);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 43344);
											match s {
												[] => true,
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
									super::push(out, 42790);
									match s {
										[] => true,
										['а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 42791);
									match s {
										[] => true,
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
					super::push(out, 40056);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 41840);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 40057);
					match s {
						[] => true,
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
			super::push(out, 36820);
			match s {
				[] => true,
				['а', 'м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 41841);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 41842);
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 36821);
			match s {
				[] => true,
				['а', 'м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 41843);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 42792);
									match s {
										[] => true,
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
							super::push(out, 41844);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 42793);
									match s {
										[] => true,
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
			super::push(out, 36822);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 40058);
					match s {
						[] => true,
						['а', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лироваться.
pub(crate) fn n55473(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31950);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 36823);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 40059);
					match s {
						[] => true,
						['е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 40060);
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 40061);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 41845);
							match s {
								[] => true,
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
					super::push(out, 40062);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 41846);
							match s {
								[] => true,
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
					super::push(out, 40063);
					match s {
						[] => true,
						['т', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 40064);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 41847);
							match s {
								[] => true,
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
					super::push(out, 40065);
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 42794);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 43345);
											match s {
												[] => true,
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
		['б', s @ ..] => {
			super::push(out, 36824);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 40066);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 41848);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 42795);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 42796);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 42797);
									match s {
										[] => true,
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
					super::push(out, 40067);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 41849);
							match s {
								[] => true,
								['с', 'с', 'а', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 40068);
					match s {
						[] => true,
						['д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 42798);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 42799);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 42800);
									match s {
										[] => true,
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 42801);
									match s {
										[] => true,
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
			super::push(out, 36825);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 40069);
					match s {
						[] => true,
						['а', 'т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ш', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 40070);
					match s {
						[] => true,
						['и', 'н', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 42802);
									match s {
										[] => true,
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
					super::push(out, 40071);
					match s {
						[] => true,
						['о', 'м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 42803);
									match s {
										[] => true,
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
					super::push(out, 40072);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 41850);
							match s {
								[] => true,
								['н', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 43346);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 43639);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t32::n90142(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 40073);
					s.is_empty()
				}
				['п', 'а', 'т', 'ш', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', 'р', 'о', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', 'с', 'о', 'р', 'д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'р', 'а', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t32::n68633(s, out),
		['л', s @ ..] => {
			super::push(out, 36827);
			match s {
				[] => true,
				['а', 'т', 'с', 'н', 'и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', s @ ..] => {
					match s {
						[] => true,
						['п', 'а', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['к', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ц', 'р', 'а', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'т', 'с', 'и', 'д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'м', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t32::n68635(s, out),
		['с', s @ ..] => super::t32::n68636(s, out),
		['у', s @ ..] => super::t32::n68637(s, out),
		['ф', s @ ..] => super::t32::n68638(s, out),
		_ => false,
	}
}

/// Words ending -мироваться.
pub(crate) fn n55474(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31951);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 36832);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 40083);
					match s {
						[] => true,
						['ь', 'л', 'а', 'м', 'а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 40084);
					match s {
						[] => true,
						['ь', 'л', 'а', 'б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 42824);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 42825);
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
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 40085);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									match s {
										[] => true,
										['д', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 43641);
													match s {
														[] => true,
														['р', s @ ..] => super::t32::n90144(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['з', s @ ..] => {
													super::push(out, 43642);
													match s {
														[] => true,
														['а', s @ ..] => super::t32::n90145(s, out),
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
									super::push(out, 42826);
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
					super::push(out, 40086);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 41866);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 42827);
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
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 36833);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 40087);
					match s {
						[] => true,
						['р', 'т', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 36834);
			match s {
				[] => true,
				['а', 'р', 'ф', 'а', 'и', 'д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 36835);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 40088);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 41867);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 41868);
							match s {
								[] => true,
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
			super::push(out, 36836);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 40089);
					match s {
						[] => true,
						['п', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 36837);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 40090);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 41869);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 42828);
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
				['н', s @ ..] => {
					super::push(out, 40091);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 41870);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 40092);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 41871);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 42829);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 42830);
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
								['з', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 41872);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 42831);
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
				['с', s @ ..] => {
					super::push(out, 40093);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 41873);
							match s {
								[] => true,
								['о', 'р', 'п', 'п', 'а', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 40094);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 41874);
							match s {
								[] => true,
								['г', 'е', 'л', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => super::t32::n68645(s, out),
		['о', s @ ..] => {
			super::push(out, 36839);
			match s {
				[] => true,
				['л', 'п', 'и', 'д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', s @ ..] => super::t32::n78565(s, out),
				['т', 'а', 'н', 'а', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t33::n68647(s, out),
		['у', s @ ..] => super::t33::n68648(s, out),
		['ф', s @ ..] => super::t33::n68649(s, out),
		['ь', s @ ..] => super::t33::n68650(s, out),
		['ю', s @ ..] => super::t33::n68651(s, out),
		_ => false,
	}
}

/// Words ending -нироваться.
pub(crate) fn n55475(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31952);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 36845);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 40102);
					match s {
						[] => true,
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 40103);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 42843);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 42844);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 42845);
									match s {
										[] => true,
										['р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 42846);
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
				['п', s @ ..] => {
					super::push(out, 40104);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 41883);
							s.is_empty()
						}
						['е', 'р', 'т', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['м', 'о', 'к', 'к', 'а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 40105);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 41884);
							match s {
								[] => true,
								['э', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 40106);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 40107);
					match s {
						[] => true,
						['н', 'о', 'ф', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 40108);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 42847);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 43361);
											match s {
												[] => true,
												['е', 'ц', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 42848);
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
			super::push(out, 36846);
			match s {
				[] => true,
				['е', 'р', 'п', 'м', 'и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'с', s @ ..] => {
					match s {
						[] => true,
						['а', 'р', 'т', 'н', 'о', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'а', 'р', 'т', 'н', 'о', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 36847);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 40109);
					match s {
						[] => true,
						['б', 'и', 'л', 'о', 'м', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 40110);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 40111);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 41885);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 42849);
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
						['д', s @ ..] => {
							super::push(out, 41886);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 42850);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 41887);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 42851);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 42852);
									match s {
										[] => true,
										['н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 43362);
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
								['о', 'п', '\u{1}', s @ ..] => {
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
				['ц', s @ ..] => {
					super::push(out, 40112);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 41888);
							match s {
								[] => true,
								['н', s @ ..] => {
									match s {
										[] => true,
										['а', 'з', 'и', 'м', '\u{1}', s @ ..] => {
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t33::n68655(s, out),
		['й', 'а', 'б', 'м', 'о', 'к', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 36849);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 40119);
					match s {
						[] => true,
						['к', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t33::n68658(s, out),
		['р', s @ ..] => {
			super::push(out, 36851);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['г', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['к', 'н', 'и', 'е', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 40127);
					match s {
						[] => true,
						['т', s @ ..] => {
							match s {
								[] => true,
								['н', 'и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ь', s @ ..] => super::t33::n87467(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => super::t33::n78612(s, out),
				_ => false,
			}
		}
		['у', s @ ..] => super::t33::n68660(s, out),
		_ => false,
	}
}

/// Words ending -оироваться.
pub(crate) fn n55476(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31953);
	match s {
		[] => true,
		['в', 'н', 'о', 'к', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 40129);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 41905);
							match s {
								[] => true,
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
		['р', 'т', 'к', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пироваться.
pub(crate) fn n55477(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31954);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 36853);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 40130);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 40131);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 40132);
					match s {
						[] => true,
						['д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 42868);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 42869);
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
				['т', s @ ..] => {
					super::push(out, 40133);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 41906);
							match s {
								[] => true,
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
			super::push(out, 36854);
			match s {
				[] => true,
				['р', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ц', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 36855);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 40134);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 41907);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', 'н', 'а', 'м', 'э', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 40135);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'р', 'т', 'н', 'о', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'е', 'р', 'е', 'т', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 40136);
					match s {
						[] => true,
						['е', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'т', 'н', 'а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'е', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', 'а', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 36856);
			match s {
				[] => true,
				['а', 'т', 'е', 'л', 'е', 'т', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 36857);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 40137);
					match s {
						[] => true,
						['р', 'р', 'о', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 42870);
									match s {
										[] => true,
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
			super::push(out, 36858);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 40138);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 41908);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 42871);
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
						['н', s @ ..] => {
							super::push(out, 41909);
							match s {
								[] => true,
								['и', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['р', 'е', 'с', 'к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 43382);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 43652);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t33::n90155(s, out),
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
									super::push(out, 42872);
									match s {
										[] => true,
										['о', 'ф', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 41910);
							match s {
								[] => true,
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
					super::push(out, 40139);
					match s {
						[] => true,
						['ш', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 40140);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 41911);
							match s {
								[] => true,
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
			super::push(out, 36859);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 40141);
					match s {
						[] => true,
						['р', 'г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 42873);
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['з', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 42874);
									match s {
										[] => true,
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
			super::push(out, 36860);
			match s {
				[] => true,
				['а', 'к', 'с', 'э', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', 'з', 'у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 36861);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 40142);
					match s {
						[] => true,
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
			super::push(out, 36862);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['к', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 36863);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 40143);
					match s {
						[] => true,
						['а', s @ ..] => super::t33::n84449(s, out),
						['у', s @ ..] => super::t33::n84450(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рироваться.
pub(crate) fn n55478(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31955);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 40144);
					match s {
						[] => true,
						['к', 'е', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 40145);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 42876);
									match s {
										[] => true,
										['п', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['т', s @ ..] => {
													super::push(out, 43653);
													match s {
														[] => true,
														['о', s @ ..] => super::t33::n90156(s, out),
														_ => false,
													}
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
						['м', 'о', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 40146);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', 'м', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 36864);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 40147);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 41913);
							match s {
								[] => true,
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
					super::push(out, 40148);
					match s {
						[] => true,
						['л', 'а', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ф', 'е', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 36865);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', 'е', 'т', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'с', 'е', 'р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 42877);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 43383);
											match s {
												[] => true,
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
		['г', s @ ..] => {
			super::push(out, 36866);
			match s {
				[] => true,
				['е', 'т', 'н', 'и', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 41914);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 42878);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', 'е', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 36867);
			match s {
				[] => true,
				['а', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 40149);
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 40150);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 41915);
							match s {
								[] => true,
								['л', 'а', 'к', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 36868);
			match s {
				[] => true,
				['д', 'о', 'м', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['л', 'е', 'с', 'к', 'а', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['м', 'о', 'л', 'г', 'а', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', 'е', 'г', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 41916);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 42879);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 40151);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 41917);
							match s {
								[] => true,
								['е', 'т', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 41918);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 42880);
									match s {
										[] => true,
										['к', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 43654);
													s.is_empty()
												}
												['с', s @ ..] => {
													super::push(out, 43655);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t33::n90157(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 43384);
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
							super::push(out, 41919);
							match s {
								[] => true,
								['к', 'е', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 40152);
					match s {
						[] => true,
						['и', 'л', 'л', 'а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', 'е', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ц', 'а', 'м', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t33::n68680(s, out),
		['о', s @ ..] => super::t33::n68681(s, out),
		['т', s @ ..] => super::t33::n68682(s, out),
		['у', s @ ..] => super::t33::n68683(s, out),
		['ф', s @ ..] => {
			super::push(out, 36872);
			match s {
				[] => true,
				['и', s @ ..] => super::t33::n78678(s, out),
				['о', s @ ..] => super::t33::n78679(s, out),
				_ => false,
			}
		}
		['э', s @ ..] => super::t33::n68685(s, out),
		['ю', 'б', 'р', 'а', 'к', '\u{1}', s @ ..] => super::t33::n68686(s, out),
		_ => false,
	}
}

/// Words ending -сироваться.
pub(crate) fn n55479(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31956);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 36874);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 40161);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 41927);
							match s {
								[] => true,
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
					super::push(out, 40162);
					match s {
						[] => true,
						['р', 'е', 'т', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', 'а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 36875);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 40163);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 40164);
					match s {
						[] => true,
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
			super::push(out, 36876);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 40165);
					match s {
						[] => true,
						['л', 'е', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					match s {
						[] => true,
						['д', 'н', 'и', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 42893);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 43392);
											match s {
												[] => true,
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
						['л', 'п', 'м', 'о', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', 'н', 'а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'ф', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 41928);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 42894);
									match s {
										[] => true,
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
					super::push(out, 40166);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 42895);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 42896);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 43393);
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
									super::push(out, 42897);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 43394);
											match s {
												[] => true,
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
		['н', s @ ..] => {
			super::push(out, 36877);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 40167);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 41929);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 42898);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 43395);
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
						['д', s @ ..] => {
							super::push(out, 41930);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 42899);
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
						['л', s @ ..] => {
							super::push(out, 41931);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', 'б', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['з', s @ ..] => {
											super::push(out, 43396);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 43661);
													match s {
														[] => true,
														['р', '\u{1}', s @ ..] => super::t33::n90162(s, out),
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
													super::push(out, 43662);
													s.is_empty()
												}
												['и', s @ ..] => {
													super::push(out, 43663);
													match s {
														[] => true,
														['д', '\u{1}', s @ ..] => super::t33::n90163(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => super::t33::n84518(s, out),
						['ю', s @ ..] => super::t33::n84519(s, out),
						_ => false,
					}
				}
				['е', s @ ..] => super::t33::n78690(s, out),
				['о', s @ ..] => super::t33::n78691(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => super::t33::n68692(s, out),
		['с', s @ ..] => super::t33::n68693(s, out),
		['у', s @ ..] => super::t33::n68694(s, out),
		['ь', 'л', 'у', 'м', 'э', '\u{1}', s @ ..] => super::t33::n68695(s, out),
		['ю', 'д', 'о', 'р', 'п', '\u{1}', s @ ..] => super::t33::n68696(s, out),
		_ => false,
	}
}

/// Words ending -тироваться.
pub(crate) fn n55480(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31957);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 36881);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 36882);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 40175);
					match s {
						[] => true,
						['е', 'д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 42910);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 43406);
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
					super::push(out, 40176);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 41945);
							match s {
								[] => true,
								['к', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 41946);
							match s {
								[] => true,
								['р', 'г', 'а', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 40177);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'т', 'н', 'а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', 'и', 'л', 'и', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 40178);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 41947);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 41948);
							match s {
								[] => true,
								['з', 'а', 'к', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 41949);
							match s {
								[] => true,
								['о', 'ф', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 43407);
											s.is_empty()
										}
										['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
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
						['ь', s @ ..] => {
							super::push(out, 41950);
							match s {
								[] => true,
								['л', 'о', 'к', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', 'л', 'п', 'с', 'к', 'э', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 40179);
					match s {
						[] => true,
						['е', 'л', 'е', 'т', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['э', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', 'д', 'и', 'г', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'с', 'н', 'о', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 40180);
					match s {
						[] => true,
						['л', 'п', 'с', 'к', 'э', s @ ..] => {
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
				['ф', 'с', 'о', 'ф', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ш', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 36883);
			match s {
				[] => true,
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 40181);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 41951);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 42911);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 43408);
											match s {
												[] => true,
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
									super::push(out, 42912);
									match s {
										[] => true,
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
							super::push(out, 41952);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'б', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['т', 'э', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', 'а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 40182);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 41953);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 42913);
									match s {
										[] => true,
										['т', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 41954);
							match s {
								[] => true,
								['к', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 40183);
					match s {
						[] => true,
						['о', 'р', 'п', 'м', 'о', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 42914);
									match s {
										[] => true,
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
				['п', s @ ..] => super::t33::n78716(s, out),
				['р', s @ ..] => super::t33::n78717(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t33::n68700(s, out),
		['й', 'е', 'ф', 'р', 'о', 'ф', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['к', s @ ..] => super::t33::n68702(s, out),
		['н', s @ ..] => super::t33::n68703(s, out),
		['о', s @ ..] => super::t33::n68704(s, out),
		['п', s @ ..] => super::t33::n68705(s, out),
		['р', s @ ..] => super::t33::n68706(s, out),
		['с', s @ ..] => super::t33::n68707(s, out),
		['у', s @ ..] => super::t33::n68708(s, out),
		['ь', s @ ..] => super::t33::n68709(s, out),
		['ю', s @ ..] => super::t33::n68710(s, out),
		_ => false,
	}
}

/// Words ending -уироваться.
pub(crate) fn n55481(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31958);
	match s {
		[] => true,
		['б', 'а', 'т', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['в', 'а', 'з', 'е', 'д', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['д', 'а', 'р', 'г', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 36894);
			match s {
				[] => true,
				['а', 'в', 'э', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', 'и', 'с', 'н', 'и', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 36895);
			match s {
				[] => true,
				['т', 'с', 'н', 'о', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 42955);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 43448);
											match s {
												[] => true,
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
							super::push(out, 42003);
							match s {
								[] => true,
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
			super::push(out, 36896);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 40220);
					match s {
						[] => true,
						['т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 42956);
									match s {
										[] => true,
										['н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 42957);
									match s {
										[] => true,
										['р', 'э', 'а', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 42958);
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
				['и', s @ ..] => {
					super::push(out, 40221);
					match s {
						[] => true,
						['т', 'с', s @ ..] => {
							match s {
								[] => true,
								['н', 'о', 'к', '\u{1}', s @ ..] => {
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
				['н', 'е', 'ц', 'к', 'а', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фироваться.
pub(crate) fn n55482(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31959);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 36897);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 42004);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 42959);
									match s {
										[] => true,
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
							super::push(out, 42005);
							match s {
								[] => true,
								['е', 'л', 'е', 'т', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									match s {
										[] => true,
										['и', 'л', 'б', 'и', 'б', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['к', 'и', 'с', 'к', 'е', 'л', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['л', s @ ..] => {
											super::push(out, 43449);
											match s {
												[] => true,
												['к', 'е', 'т', 'с', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['о', 'г', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											match s {
												[] => true,
												['е', 'т', 'с', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t33::n90195(s, out),
														['а', s @ ..] => super::t33::n90196(s, out),
														_ => false,
													}
												}
												['о', 'ф', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', 'т', 'к', 'е', 'п', 'с', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 43450);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 43691);
													match s {
														[] => true,
														['л', '\u{1}', s @ ..] => super::t33::n90197(s, out),
														_ => false,
													}
												}
												['к', s @ ..] => {
													super::push(out, 43692);
													match s {
														[] => true,
														['е', 'г', '\u{1}', s @ ..] => super::t33::n90198(s, out),
														_ => false,
													}
												}
												['о', s @ ..] => {
													super::push(out, 43693);
													match s {
														[] => true,
														['ф', s @ ..] => super::t33::n90199(s, out),
														_ => false,
													}
												}
												['р', s @ ..] => {
													super::push(out, 43694);
													match s {
														[] => true,
														['а', 'к', '\u{1}', s @ ..] => super::t33::n90200(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 36898);
			match s {
				[] => true,
				['р', 'т', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 42006);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 42007);
							match s {
								[] => true,
								['е', 'п', 'и', 'г', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 42008);
							match s {
								[] => true,
								['о', 'п', 'а', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', 'м', 'е', 'д', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 36899);
			match s {
				[] => true,
				['о', 'м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'н', 'а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', 'л', 'у', 'с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -цироваться.
pub(crate) fn n55483(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31960);
	match s {
		[] => true,
		['е', s @ ..] => {
			match s {
				[] => true,
				['з', 'е', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 40222);
					match s {
						[] => true,
						['р', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 42960);
									match s {
										[] => true,
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
				['ъ', s @ ..] => {
					super::push(out, 40223);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 42009);
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
		['и', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 40224);
					match s {
						[] => true,
						['н', 'и', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
				['к', s @ ..] => {
					super::push(out, 40225);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 42010);
							match s {
								[] => true,
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
					super::push(out, 40226);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 42961);
									match s {
										[] => true,
										['р', 'о', 'м', 'а', 'с', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['м', s @ ..] => {
									super::push(out, 42962);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 43451);
											match s {
												[] => true,
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
									super::push(out, 42963);
									match s {
										[] => true,
										['а', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 42964);
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
				['р', s @ ..] => {
					super::push(out, 40227);
					match s {
						[] => true,
						['б', 'у', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'а', 'м', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => super::t33::n78784(s, out),
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 36900);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 42015);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 42972);
									match s {
										[] => true,
										['ф', 'ф', 'и', 'д', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 42016);
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
		['о', s @ ..] => super::t33::n68726(s, out),
		['с', 'о', 'н', 'г', 'о', 'к', 'е', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['у', s @ ..] => super::t33::n68728(s, out),
		_ => false,
	}
}

/// Words ending -чироваться.
pub(crate) fn n55484(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31961);
	match s {
		[] => true,
		['ь', 'л', 'у', 'м', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 40228);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 42018);
							match s {
								[] => true,
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

/// Words ending -широваться.
pub(crate) fn n55485(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31962);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 36903);
			match s {
				[] => true,
				['к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', 'а', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 36904);
			match s {
				[] => true,
				['л', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 42019);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 42974);
									match s {
										[] => true,
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
				['т', 'е', 'ф', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 40229);
					match s {
						[] => true,
						['а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'р', 'к', 'и', 'м', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 36905);
			match s {
				[] => true,
				['и', 'м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 42020);
							match s {
								[] => true,
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
			super::push(out, 36906);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 42021);
							match s {
								[] => true,
								['е', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 42022);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 42975);
									match s {
										[] => true,
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
			super::push(out, 36907);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 40230);
					match s {
						[] => true,
						['ф', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 42976);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 42977);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 36908);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 40231);
					match s {
						[] => true,
						['е', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 42023);
							match s {
								[] => true,
								['р', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 43464);
											s.is_empty()
										}
										['а', 'з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['д', 'о', 'п', '\u{1}', s @ ..] => {
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 36909);
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

/// Words ending -ьироваться.
pub(crate) fn n55486(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31963);
	s.is_empty()
}

/// Words ending -юироваться.
pub(crate) fn n55487(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31964);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 36910);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 40232);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 36911);
			match s {
				[] => true,
				['в', 'р', 'е', 'т', 'н', 'и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word переаттестоваться.
pub(crate) fn n89931(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word легализовываться.
pub(crate) fn n87781(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word адвербиализовываться.
pub(crate) fn n87782(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -мобилизовываться.
pub(crate) fn n84856(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', 'д', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кристаллизовываться.
pub(crate) fn n84857(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['с', 'а', 'р', '\u{1}', s @ ..] => {
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

/// Words ending -организовываться.
pub(crate) fn n79040(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', 'р', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', 'е', 'д', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ризовываться.
pub(crate) fn n79041(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['е', 'т', 'к', 'а', 'р', 'а', 'х', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', 'к', 'е', 'л', 'э', 'а', 'н', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word пропемзовываться.
pub(crate) fn n68949(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word ошлюзовываться.
pub(crate) fn n68950(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ковываться.
pub(crate) fn n55607(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['л', 'ш', s @ ..] => {
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
				['н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', s @ ..] => {
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'а', 'р', '\u{1}', s @ ..] => {
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
				['р', 'б', s @ ..] => {
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
						['з', 'а', 'р', '\u{1}', s @ ..] => {
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
				['т', 'а', '\u{1}', s @ ..] => {
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
		['и', s @ ..] => {
			match s {
				[] => true,
				['л', 'б', 'у', 'п', s @ ..] => {
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
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'и', 'р', 'к', 'с', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', 'о', 'т', s @ ..] => {
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
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
		['н', s @ ..] => {
			match s {
				[] => true,
				['е', 'р', 'е', 'ч', 'с', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', s @ ..] => {
					match s {
						[] => true,
						['ц', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ш', 'а', 'н', '\u{1}', s @ ..] => {
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
		['р', s @ ..] => {
			match s {
				[] => true,
				['а', 'п', s @ ..] => {
					match s {
						[] => true,
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'к', 'о', '\u{1}', s @ ..] => {
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
		['у', 'т', 'ш', s @ ..] => {
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
		['ы', s @ ..] => {
			match s {
				[] => true,
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'с', s @ ..] => {
					match s {
						[] => true,
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
				_ => false,
			}
		}
		['ю', 'т', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -ловываться.
pub(crate) fn n55608(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32034);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
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
						_ => false,
					}
				}
				['в', 'б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 40300);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 42051);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 42986);
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
		['е', 'ц', s @ ..] => {
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
				['с', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', 'е', 'т', 'с', 'с', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мовываться.
pub(crate) fn n55609(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['р', 'о', 'ф', s @ ..] => {
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
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', 'и', 'р', 'а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ь', 'л', 'е', 'ш', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -новываться.
pub(crate) fn n55610(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['г', 'и', 'с', 'с', 'а', s @ ..] => {
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
		['е', 'м', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', 'к', 'э', s @ ..] => {
							match s {
								[] => true,
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
								_ => false,
							}
						}
						['н', 'з', s @ ..] => {
							match s {
								[] => true,
								['д', 'е', 'р', 'п', '\u{1}', s @ ..] => {
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
				['и', s @ ..] => {
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
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
				['ы', 'з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
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
				['р', 'а', 'м', 'а', s @ ..] => {
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
				['ш', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['п', 'м', 'о', 'к', s @ ..] => {
					match s {
						[] => true,
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', 'о', 'б', 'з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', 'е', 'д', s @ ..] => {
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
		['с', 'о', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
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

/// Words ending -повываться.
pub(crate) fn n55611(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['е', 'р', 'к', 'с', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['м', 'а', 'т', 'ш', s @ ..] => {
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

/// Words ending -ровываться.
pub(crate) fn n55612(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', 'ч', s @ ..] => {
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
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
		['б', 'и', 'л', 'а', 'к', 'т', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['д', 'а', 'к', 'с', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['з', 'е', 'р', 'ф', 'т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['м', s @ ..] => {
					match s {
						[] => true,
						['о', 'н', s @ ..] => {
							match s {
								[] => true,
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', 'н', s @ ..] => {
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
				['б', 'м', 'о', 'л', 'п', s @ ..] => {
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
				['в', 'а', 'р', 'г', 'з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'н', s @ ..] => {
					match s {
						[] => true,
						['а', 'м', 'о', 'к', s @ ..] => {
							match s {
								[] => true,
								['и', 'р', 'п', '\u{1}', s @ ..] => {
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
						['у', 'м', 'б', 'о', s @ ..] => {
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
				['ж', 'о', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', 'н', 'о', 'р', 'б', 'а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['к', s @ ..] => {
					match s {
						[] => true,
						['а', 'л', s @ ..] => {
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
								['т', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', 'п', 'с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'а', 'м', 'а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					match s {
						[] => true,
						['б', 'е', 'м', s @ ..] => {
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
						['о', 'п', s @ ..] => {
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
								['с', 'а', 'р', '\u{1}', s @ ..] => {
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
				['м', s @ ..] => {
					match s {
						[] => true,
						['и', 'р', 'г', s @ ..] => {
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
								['з', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', 'о', 'ф', s @ ..] => {
							match s {
								[] => true,
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => super::t33::n87830(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => super::t33::n79115(s, out),
				['п', s @ ..] => super::t33::n79116(s, out),
				['с', s @ ..] => super::t33::n79117(s, out),
				['т', s @ ..] => super::t33::n79118(s, out),
				['ш', 'р', 'а', 'ф', 'а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', 'в', s @ ..] => super::t33::n68985(s, out),
		['т', s @ ..] => super::t33::n68986(s, out),
		['у', s @ ..] => super::t33::n68987(s, out),
		['ф', 'и', s @ ..] => super::t33::n68988(s, out),
		_ => false,
	}
}

/// Words ending -совываться.
pub(crate) fn n55613(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32035);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['л', 'г', 'о', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', s @ ..] => {
					match s {
						[] => true,
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
				['т', s @ ..] => {
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
				['ф', 'с', 'а', 'р', '\u{1}', s @ ..] => {
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
		['е', 'р', s @ ..] => {
			match s {
				[] => true,
				['д', 'а', s @ ..] => {
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['и', 'р', s @ ..] => {
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
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
				_ => false,
			}
		}
		['к', 'е', 'л', 'п', 'м', 'о', 'к', 'с', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['л', 'о', s @ ..] => {
					match s {
						[] => true,
						['г', 'е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', 'с', s @ ..] => {
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
				['е', 'р', 'п', s @ ..] => {
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
						['и', 'р', 'п', '\u{1}', s @ ..] => {
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
		['у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 36993);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 42052);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 42987);
									match s {
										[] => true,
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
		['ю', 'л', s @ ..] => {
			match s {
				[] => true,
				['п', 'и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ф', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -товываться.
pub(crate) fn n55614(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['и', 'д', 'е', 'р', 'к', 'к', 'а', s @ ..] => {
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
		['й', 'а', 'н', 'и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['к', s @ ..] => {
			match s {
				[] => true,
				['а', 'р', 'т', s @ ..] => {
					match s {
						[] => true,
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', 'о', 'к', 'а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', 'л', 'п', 'м', 'о', 'к', s @ ..] => {
					match s {
						[] => true,
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'а', 'р', '\u{1}', s @ ..] => {
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
				['и', 'д', s @ ..] => {
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
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
		['л', 'о', 'б', 'з', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['н', s @ ..] => {
			match s {
				[] => true,
				['а', 'к', s @ ..] => {
					match s {
						[] => true,
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
				['е', s @ ..] => {
					match s {
						[] => true,
						['м', 'у', 'р', 'т', 'с', 'н', 'и', 'е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'а', 'п', 'а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'б', s @ ..] => {
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
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
						_ => false,
					}
				}
				['у', s @ ..] => {
					match s {
						[] => true,
						['п', 'ш', 'а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'г', s @ ..] => {
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
				['а', 'в', 'ш', s @ ..] => {
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
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'п', 'с', s @ ..] => {
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
						['о', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', 'б', s @ ..] => {
					match s {
						[] => true,
						['з', 'а', 'р', '\u{1}', s @ ..] => {
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
		['с', s @ ..] => {
			match s {
				[] => true,
				['а', 'л', 'п', s @ ..] => {
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
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					match s {
						[] => true,
						['р', 'а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', s @ ..] => {
							match s {
								[] => true,
								['о', 'р', 'п', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['т', 'а', s @ ..] => {
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
				['к', 'е', 'т', 'д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			match s {
				[] => true,
				['а', 'р', 'ф', 'а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'л', 'ш', s @ ..] => {
					match s {
						[] => true,
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
		['ч', 'а', 'м', 'з', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фовываться.
pub(crate) fn n55615(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', 'р', 'т', 'ш', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['л', 'ш', s @ ..] => {
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
				['р', 'а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ховываться.
pub(crate) fn n55616(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', 'р', 'т', 'с', 'е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', 'р', 'т', 'ш', 'а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -цовываться.
pub(crate) fn n55617(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['б', 'у', 'р', s @ ..] => {
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', 'и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', 'л', s @ ..] => {
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
		['н', s @ ..] => {
			match s {
				[] => true,
				['а', 'т', s @ ..] => {
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
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
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'в', 'с', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', 'о', 'т', s @ ..] => {
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
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
		['ь', 'л', s @ ..] => {
			match s {
				[] => true,
				['а', 'в', 'з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
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

/// The word разбушовываться.
pub(crate) fn n55618(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word переизбыточествовать.
pub(crate) fn n90221(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word благопоспешествовать.
pub(crate) fn n89936(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word запутешествовать.
pub(crate) fn n89937(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word пороскошествовать.
pub(crate) fn n89938(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ушествовать.
pub(crate) fn n62807(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34700);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 38493);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 42199);
							match s {
								[] => true,
								['а', 'л', 'б', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 43473);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 43474);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 43707);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t33::n90222(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 43475);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 43708);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t33::n90223(s, out),
														_ => false,
													}
												}
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
							super::push(out, 42200);
							match s {
								[] => true,
								['и', 'л', 'е', 'в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 42201);
							match s {
								[] => true,
								['а', 'м', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 43476);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 43709);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t33::n90224(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 43477);
											match s {
												[] => true,
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
							super::push(out, 42202);
							match s {
								[] => true,
								['я', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 42203);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 43027);
									match s {
										[] => true,
										['а', 'р', 'к', 'е', 'р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 38494);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 40802);
					match s {
						[] => true,
						['л', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 38495);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 40803);
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

/// The word пенсионерствовать.
pub(crate) fn n89945(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word миссионерствовать.
pub(crate) fn n89946(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word комиссионерствовать.
pub(crate) fn n89947(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -терствовать.
pub(crate) fn n62907(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34777);
	match s {
		[] => true,
		['е', s @ ..] => {
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 40888);
					match s {
						[] => true,
						['ш', 'у', 'м', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 40889);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 42239);
							match s {
								[] => true,
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
			super::push(out, 38590);
			match s {
				[] => true,
				['е', 'ж', 'о', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 38591);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 40890);
					match s {
						[] => true,
						['г', 'х', 'у', 'б', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 38592);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 40891);
					match s {
						[] => true,
						['л', 'о', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 38593);
			match s {
				[] => true,
				['з', 'у', 'б', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 38594);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 38595);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 40892);
					match s {
						[] => true,
						['н', 'и', 'м', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шаферствовать.
pub(crate) fn n81619(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40893);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -черствовать.
pub(crate) fn n62909(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34779);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 38597);
			match s {
				[] => true,
				['е', 'п', 'с', 'и', 'д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шерствовать.
pub(crate) fn n62910(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34780);
	match s {
		[] => true,
		['у', 'к', 'а', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьерствовать.
pub(crate) fn n62911(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34781);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 38598);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 40894);
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
		['н', s @ ..] => {
			super::push(out, 38599);
			match s {
				[] => true,
				['о', 'к', 'а', 'р', 'б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 42240);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 43044);
									match s {
										[] => true,
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

/// The word поораторствовать.
pub(crate) fn n89948(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word регистраторствовать.
pub(crate) fn n89949(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word администраторствовать.
pub(crate) fn n89950(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -таторствовать.
pub(crate) fn n81640(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 42248);
			match s {
				[] => true,
				['и', 'д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 42249);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 43048);
					match s {
						[] => true,
						['м', 'и', 'р', 'е', 'п', 'с', 'к', 'э', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -авторствовать.
pub(crate) fn n74418(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 40902);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 42250);
					match s {
						[] => true,
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

/// Words ending -иторствовать.
pub(crate) fn n74419(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 40903);
			match s {
				[] => true,
				['о', 'п', 'м', 'о', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 40904);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 42251);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 42252);
					match s {
						[] => true,
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

/// Words ending -кторствовать.
pub(crate) fn n74420(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38610);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['д', 'е', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 42253);
					match s {
						[] => true,
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
			super::push(out, 40905);
			match s {
				[] => true,
				['п', 'с', 'н', 'и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нторствовать.
pub(crate) fn n74421(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38611);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 40906);
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

/// Words ending -яторствовать.
pub(crate) fn n74422(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38612);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 40907);
			match s {
				[] => true,
				['и', 'м', 'и', 'с', 'с', 'а', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -заседательствовать.
pub(crate) fn n90226(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43789);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -председательствовать.
pub(crate) fn n90227(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 43815);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 43818);
					match s {
						[] => true,
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

/// Words ending -ожелательствовать.
pub(crate) fn n85867(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['г', 'а', 'л', 'б', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 43058);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 43484);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', 'б', 'о', 'д', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 43485);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 43713);
							match s {
								[] => true,
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

/// The word предпринимательствовать.
pub(crate) fn n85868(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word старательствовать.
pub(crate) fn n85869(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -сательствовать.
pub(crate) fn n85870(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42268);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 43059);
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

/// Words ending -тательствовать.
pub(crate) fn n85871(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 43060);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 43486);
					match s {
						[] => true,
						['о', 'н', 'д', 'з', 'а', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', 'д', 'е', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word злопыхательствовать.
pub(crate) fn n85872(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word соглашательствовать.
pub(crate) fn n85873(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -етельствовать.
pub(crate) fn n81712(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40946);
	match s {
		[] => true,
		['д', s @ ..] => {
			match s {
				[] => true,
				['и', 'в', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 43487);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 43488);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 43714);
									match s {
										[] => true,
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
							super::push(out, 43489);
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
				['о', s @ ..] => {
					super::push(out, 43061);
					match s {
						[] => true,
						['г', 'а', 'л', 'б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 43715);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 43716);
									match s {
										[] => true,
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

/// Words ending -ительствовать.
pub(crate) fn n81713(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40947);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 42269);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 43062);
					match s {
						[] => true,
						['р', 'г', '\u{1}', s @ ..] => {
							s.is_empty()
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
				['а', s @ ..] => {
					super::push(out, 43063);
					match s {
						[] => true,
						['т', 'с', 'д', 'е', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', 'р', 'к', 'о', 'п', '\u{1}', s @ ..] => {
					super::push(out, 43064);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 43065);
					match s {
						[] => true,
						['р', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 43066);
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['д', 'е', 'р', 'п', '\u{1}', s @ ..] => {
									super::push(out, 43717);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 43718);
									match s {
										[] => true,
										['к', 'у', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 43490);
							match s {
								[] => true,
								['с', 'и', 'н', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 43491);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 43719);
									match s {
										[] => true,
										['ы', 'т', 'с', 'у', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 43492);
							match s {
								[] => true,
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
			super::push(out, 42270);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 43067);
					match s {
						[] => true,
						['ч', 'о', 'с', '\u{1}', s @ ..] => {
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
				['з', s @ ..] => {
					super::push(out, 43068);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 43069);
					match s {
						[] => true,
						['в', 'т', 'о', 'г', 'а', 'л', 'б', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 43070);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 43493);
							match s {
								[] => true,
								['м', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 42271);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 43071);
					match s {
						[] => true,
						['н', 'о', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 42272);
			match s {
				[] => true,
				['с', s @ ..] => {
					match s {
						[] => true,
						['е', 'м', 'в', 'о', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', 'п', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', 'в', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 42273);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 43072);
					match s {
						[] => true,
						['п', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 43073);
					match s {
						[] => true,
						['т', 'с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 43074);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 43494);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 43720);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 43721);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 43790);
											match s {
												[] => true,
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
							super::push(out, 43495);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 43722);
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

/// Words ending -ятельствовать.
pub(crate) fn n81714(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40948);
	s.is_empty()
}

/// The word заидеологизировать.
pub(crate) fn n89977(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -циализировать.
pub(crate) fn n82046(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 42344);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 43100);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 43725);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 43726);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 43727);
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
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 42345);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 43101);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 42346);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 43102);
					match s {
						[] => true,
						['т', 'с', 'б', 'у', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 42347);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 43103);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 42348);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 43104);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 42349);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 43105);
					match s {
						[] => true,
						['и', 'т', 'с', 'н', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кализировать.
pub(crate) fn n74962(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38898);
	match s {
		[] => true,
		['и', s @ ..] => {
			match s {
				[] => true,
				['д', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', 'к', 'е', 'л', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['л', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мализировать.
pub(crate) fn n74963(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38899);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 41091);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 42350);
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
		['р', s @ ..] => {
			super::push(out, 41092);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ф', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нализировать.
pub(crate) fn n74964(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38900);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 42351);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 42352);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 43106);
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
		['г', s @ ..] => {
			super::push(out, 41093);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 42353);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 43503);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 43728);
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
		['и', s @ ..] => {
			super::push(out, 41094);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 42354);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 43107);
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
				['м', s @ ..] => {
					super::push(out, 42355);
					match s {
						[] => true,
						['и', 'р', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 42356);
					match s {
						[] => true,
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
			super::push(out, 41095);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 42357);
					match s {
						[] => true,
						['г', 'е', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'с', 'е', 'ф', 'о', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ц', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 43504);
									match s {
										[] => true,
										['н', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['е', s @ ..] => {
													super::push(out, 43791);
													match s {
														[] => true,
														['д', '\u{1}', s @ ..] => super::t33::n90324(s, out),
														['р', '\u{1}', s @ ..] => super::t33::n90325(s, out),
														_ => false,
													}
												}
												['р', 'е', 'т', 'н', 'и', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['к', 'н', 'у', 'ф', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 43505);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 43729);
											match s {
												[] => true,
												['т', 'с', 'б', 'у', 'с', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 43506);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 43730);
											match s {
												[] => true,
												['и', 'т', 'с', 'н', 'и', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 42358);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 43108);
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
		_ => false,
	}
}

/// Words ending -пализировать.
pub(crate) fn n74965(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38901);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 41096);
			match s {
				[] => true,
				['ц', 'и', 'н', 'у', 'м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рализировать.
pub(crate) fn n74966(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38902);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 41097);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 42359);
					match s {
						[] => true,
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
			super::push(out, 41098);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 42360);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 43109);
							match s {
								[] => true,
								['л', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 42361);
					match s {
						[] => true,
						['е', 'г', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'м', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 41099);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 42362);
					match s {
						[] => true,
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
			super::push(out, 41100);
			match s {
				[] => true,
				['а', 'е', 'т', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['й', 'е', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сализировать.
pub(crate) fn n74967(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38903);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 41101);
			match s {
				[] => true,
				['е', 'в', 'и', 'н', 'у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тализировать.
pub(crate) fn n74968(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38904);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 41102);
			match s {
				[] => true,
				['к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['л', 'а', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 41103);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 42363);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 42364);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 43110);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 42365);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 43111);
							match s {
								[] => true,
								['к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 43731);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 43792);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t33::n90326(s, out),
														_ => false,
													}
												}
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
							super::push(out, 43112);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 41104);
			match s {
				[] => true,
				['е', 'м', 'у', 'р', 'т', 'с', 'н', 'и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уализировать.
pub(crate) fn n74969(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38905);
	match s {
		[] => true,
		['д', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 42366);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 42367);
					match s {
						[] => true,
						['в', 'и', 'д', 'н', 'и', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', 'и', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 41105);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 42368);
					match s {
						[] => true,
						['е', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 41106);
			match s {
				[] => true,
				['и', 'т', 'с', 'н', 'и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 42369);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 43113);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 43114);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 43507);
									match s {
										[] => true,
										['л', 'е', 'т', 'н', 'и', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', 'е', 'ц', 'к', 'а', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', 'е', 'ц', 'н', 'о', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', 'и', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', 'к', 'е', 'т', 'н', 'о', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -елизировать.
pub(crate) fn n63547(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35175);
	match s {
		[] => true,
		['д', 'е', 'г', '\u{1}', s @ ..] => {
			super::push(out, 38906);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 38907);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 41107);
					match s {
						[] => true,
						['р', 'а', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', 'к', 'и', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', 'р', 'а', 'к', 'е', 'д', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -илизировать.
pub(crate) fn n63548(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35176);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 38908);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 41108);
					match s {
						[] => true,
						['т', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 43115);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 43508);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['е', 'д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'р', 'и', 'г', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', 'д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 41109);
					match s {
						[] => true,
						['с', 'н', 'е', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 43116);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 43509);
											match s {
												[] => true,
												['о', 'ф', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 41110);
					match s {
						[] => true,
						['м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['м', 'и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'т', 'в', 'а', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 38909);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 41111);
					match s {
						[] => true,
						['с', 'к', 'а', 'м', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', 'у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 38910);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 42370);
							match s {
								[] => true,
								['л', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 42371);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 43117);
									match s {
										[] => true,
										['и', 'г', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ллизировать.
pub(crate) fn n63549(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35177);
	match s {
		[] => true,
		['а', 'т', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 41112);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 42372);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 41113);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 42373);
							match s {
								[] => true,
								['р', 'к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 43510);
											s.is_empty()
										}
										['ы', s @ ..] => {
											super::push(out, 43511);
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -олизировать.
pub(crate) fn n63550(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35178);
	match s {
		[] => true,
		['б', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 42374);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 43118);
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
							super::push(out, 42375);
							match s {
								[] => true,
								['е', 'м', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 41114);
					match s {
						[] => true,
						['е', 'п', 'и', 'г', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 38911);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 41115);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 38912);
			match s {
				[] => true,
				['о', 'к', 'л', 'а', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 38913);
			match s {
				[] => true,
				['р', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 38914);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 41116);
					match s {
						[] => true,
						['н', 'о', 'м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', 'т', 'н', 'а', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 38915);
			match s {
				[] => true,
				['д', 'и', 'г', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 38916);
			match s {
				[] => true,
				['в', 'а', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', 'а', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word африканизировать.
pub(crate) fn n89991(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word отвулканизировать.
pub(crate) fn n89992(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -украинизировать.
pub(crate) fn n75015(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', 'д', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word лютеинизировать.
pub(crate) fn n75016(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -линизировать.
pub(crate) fn n75017(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38939);
	match s {
		[] => true,
		['г', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 41135);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 42381);
					match s {
						[] => true,
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

/// The word феминизировать.
pub(crate) fn n75018(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -тинизировать.
pub(crate) fn n75019(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 41136);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 43126);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 43514);
									match s {
										[] => true,
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
		['у', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word руинизировать.
pub(crate) fn n75020(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word машинизировать.
pub(crate) fn n75021(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -онизировать.
pub(crate) fn n63559(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35187);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 38940);
			match s {
				[] => true,
				['р', 'а', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 38941);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 41137);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 38942);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 41138);
					match s {
						[] => true,
						['р', 'э', 'а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 41139);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 42382);
							match s {
								[] => true,
								['л', 'о', 'в', 'е', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 38943);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 41140);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 42383);
							match s {
								[] => true,
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
			super::push(out, 38944);
			match s {
				[] => true,
				['б', 'а', 'ш', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'д', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 38945);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 41141);
					match s {
						[] => true,
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 41142);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 42384);
							match s {
								[] => true,
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
					super::push(out, 41143);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 38946);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 38947);
			match s {
				[] => true,
				['е', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 38948);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 41144);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 42385);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 43127);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 43128);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 43515);
											match s {
												[] => true,
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
					super::push(out, 41145);
					match s {
						[] => true,
						['н', 'и', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 43129);
									match s {
										[] => true,
										['д', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 43130);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 43516);
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
				['ы', s @ ..] => {
					super::push(out, 41146);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 42386);
							match s {
								[] => true,
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
			super::push(out, 38949);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 38950);
			match s {
				[] => true,
				['е', 'л', 'е', 'т', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['м', 'и', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рнизировать.
pub(crate) fn n63560(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35188);
	match s {
		[] => true,
		['е', 'т', 'с', 'е', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -унизировать.
pub(crate) fn n63561(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35189);
	match s {
		[] => true,
		['м', 'м', 'о', 'к', 'е', 'д', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хнизировать.
pub(crate) fn n63562(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35190);
	match s {
		[] => true,
		['е', 'т', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 41147);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 41148);
					match s {
						[] => true,
						['л', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -янизировать.
pub(crate) fn n63563(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35191);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 38951);
			match s {
				[] => true,
				['а', 'л', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word затерроризировать.
pub(crate) fn n89996(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -етризировать.
pub(crate) fn n75059(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 41160);
			match s {
				[] => true,
				['л', 'е', 'б', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 41161);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 42391);
					match s {
						[] => true,
						['р', 'а', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юризировать.
pub(crate) fn n63571(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35199);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 38967);
			match s {
				[] => true,
				['а', 'и', 'н', 'и', 'м', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яризировать.
pub(crate) fn n63572(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35200);
	match s {
		[] => true,
		['л', 'у', 'к', 'е', 'с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ь', 'д', 'а', 'м', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word переприватизировать.
pub(crate) fn n89997(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word климатизировать.
pub(crate) fn n86187(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42399);
	s.is_empty()
}

/// Words ending -аклиматизировать.
pub(crate) fn n86188(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42400);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -акклиматизировать.
pub(crate) fn n86189(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 43143);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 43520);
					match s {
						[] => true,
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

/// Words ending -мматизировать.
pub(crate) fn n82184(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41165);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 42401);
			match s {
				[] => true,
				['л', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оматизировать.
pub(crate) fn n82185(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41166);
	match s {
		[] => true,
		['и', 'с', 'к', 'а', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['р', s @ ..] => {
			match s {
				[] => true,
				['а', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['х', 'а', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', 'в', 'а', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', 'л', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рматизировать.
pub(crate) fn n82186(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41167);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 42402);
			match s {
				[] => true,
				['ф', 'н', 'и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -натизировать.
pub(crate) fn n75069(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38971);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 41168);
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

/// Words ending -патизировать.
pub(crate) fn n75070(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38972);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 41169);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 42403);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ратизировать.
pub(crate) fn n75071(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38973);
	match s {
		[] => true,
		['к', 'о', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 42404);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 43144);
							match s {
								[] => true,
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
					super::push(out, 42405);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 43145);
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 42406);
					match s {
						[] => true,
						['ю', 'б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 43521);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 43732);
											match s {
												[] => true,
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
									super::push(out, 43522);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 43733);
											match s {
												[] => true,
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

/// Words ending -татизировать.
pub(crate) fn n75072(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38974);
	match s {
		[] => true,
		['э', s @ ..] => {
			super::push(out, 41170);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фатизировать.
pub(crate) fn n75073(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38975);
	match s {
		[] => true,
		['с', 'о', 'ф', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -етизировать.
pub(crate) fn n63575(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35203);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 38976);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 41171);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 38977);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 41172);
					match s {
						[] => true,
						['е', 'н', 'э', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 38978);
			match s {
				[] => true,
				['а', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', 'а', 'м', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 38979);
			match s {
				[] => true,
				['р', 'е', 'г', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 42407);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 43146);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 38980);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 41173);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 42408);
							match s {
								[] => true,
								['м', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 43523);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 43524);
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
				['о', s @ ..] => {
					super::push(out, 41174);
					match s {
						[] => true,
						['м', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 41175);
					match s {
						[] => true,
						['е', 'б', 'и', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 38981);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 41176);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 42409);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 43147);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 42410);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 43148);
									match s {
										[] => true,
										['д', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 41177);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 42411);
							match s {
								[] => true,
								['т', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 43525);
											s.is_empty()
										}
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
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 38982);
			match s {
				[] => true,
				['с', 'э', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -итизировать.
pub(crate) fn n63576(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35204);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 38983);
			match s {
				[] => true,
				['о', 'м', 'м', 'о', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 38984);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 41178);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 43149);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 43526);
											match s {
												[] => true,
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
					super::push(out, 41179);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42412);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 38985);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 41180);
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
		['р', s @ ..] => {
			super::push(out, 38986);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 41181);
					match s {
						[] => true,
						['и', 'р', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 41182);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 42413);
							match s {
								[] => true,
								['с', 'о', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 38987);
			match s {
				[] => true,
				['а', 'р', 'г', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нтизировать.
pub(crate) fn n63577(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35205);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 38988);
			match s {
				[] => true,
				['л', 'т', 'а', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['м', 'о', 'р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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

/// Words ending -отизировать.
pub(crate) fn n63578(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35206);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 38989);
			match s {
				[] => true,
				['х', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 38990);
			match s {
				[] => true,
				['о', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 38991);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 41183);
					match s {
						[] => true,
						['з', 'о', 'м', 'о', 'г', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 38992);
			match s {
				[] => true,
				['р', 'а', 'н', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 42414);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 43150);
									match s {
										[] => true,
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
			super::push(out, 38993);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 41184);
					match s {
						[] => true,
						['и', 'г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 43151);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 43527);
											match s {
												[] => true,
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
			super::push(out, 38994);
			match s {
				[] => true,
				['в', 'е', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['э', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -птизировать.
pub(crate) fn n63579(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35207);
	match s {
		[] => true,
		['и', 'л', 'л', 'э', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ртизировать.
pub(crate) fn n63580(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 38995);
			match s {
				[] => true,
				['д', 'н', 'а', 'т', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 42415);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 43152);
									match s {
										[] => true,
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
							super::push(out, 42416);
							match s {
								[] => true,
								['р', 'в', 'е', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 41185);
					match s {
						[] => true,
						['е', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 38996);
			match s {
				[] => true,
				['м', 'а', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42417);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 42418);
							match s {
								[] => true,
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
					super::push(out, 41186);
					match s {
						[] => true,
						['с', 'а', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 38997);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 41187);
					match s {
						[] => true,
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

/// Words ending -этизировать.
pub(crate) fn n63581(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35208);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 38998);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 38999);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 41188);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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

/// Words ending -ютизировать.
pub(crate) fn n63582(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35209);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 39000);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word зарегламентировать.
pub(crate) fn n90250(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word деквалифицировать.
pub(crate) fn n90037(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ереквалифицировать.
pub(crate) fn n90038(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43739);
	match s {
		[] => true,
		['п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word аммонифицировать.
pub(crate) fn n86702(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -персонифицировать.
pub(crate) fn n86703(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', 'д', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рифицировать.
pub(crate) fn n75974(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39528);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 41543);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 42621);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 43233);
							match s {
								[] => true,
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
					super::push(out, 42622);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 43234);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 43563);
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
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 41544);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 42623);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 42624);
					match s {
						[] => true,
						['с', 'э', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['э', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 41545);
			match s {
				[] => true,
				['е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 42625);
					match s {
						[] => true,
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 42626);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сифицировать.
pub(crate) fn n75975(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39529);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 41546);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 42627);
					match s {
						[] => true,
						['т', s @ ..] => {
							match s {
								[] => true,
								['н', 'и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', 'к', 'э', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 41547);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 42628);
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 43564);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 43565);
									match s {
										[] => true,
										['д', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 41548);
			match s {
				[] => true,
				['а', 'л', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 43235);
							match s {
								[] => true,
								['р', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
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
						['с', s @ ..] => {
							super::push(out, 43236);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 43566);
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
				['у', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 41549);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 43237);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 43567);
									match s {
										[] => true,
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
			super::push(out, 41550);
			match s {
				[] => true,
				['л', 'а', 'ф', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 43238);
							match s {
								[] => true,
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

/// Words ending -тифицировать.
pub(crate) fn n75976(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39530);
	match s {
		[] => true,
		['а', 'р', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 42629);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 43239);
							match s {
								[] => true,
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
			super::push(out, 41551);
			match s {
				[] => true,
				['л', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 41552);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 42630);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 43240);
							match s {
								[] => true,
								['м', 'о', 'к', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 42631);
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
			super::push(out, 41553);
			match s {
				[] => true,
				['а', 'в', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 43241);
							s.is_empty()
						}
						['т', 'у', 'а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 41554);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 42632);
					match s {
						[] => true,
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
			super::push(out, 41555);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 41556);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 42633);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 43242);
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
					super::push(out, 42634);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 43243);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цифицировать.
pub(crate) fn n75977(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39531);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 41557);
			match s {
				[] => true,
				['н', 'е', 'д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 41558);
			match s {
				[] => true,
				['п', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нфицировать.
pub(crate) fn n64305(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35772);
	match s {
		[] => true,
		['и', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 41559);
					match s {
						[] => true,
						['р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 41560);
					match s {
						[] => true,
						['е', 'д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 43244);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 43568);
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
		_ => false,
	}
}

/// Words ending -официровать.
pub(crate) fn n64306(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35773);
	match s {
		[] => true,
		['и', 'д', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 39532);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 41561);
					match s {
						[] => true,
						['в', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', 'п', 'е', 'т', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['н', 'и', 'к', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ффицировать.
pub(crate) fn n64307(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35774);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 39533);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word засвидетельствоваться.
pub(crate) fn n90253(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -реосвидетельствоваться.
pub(crate) fn n90254(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43793);
	match s {
		[] => true,
		['е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word декриминализоваться.
pub(crate) fn n90255(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ерализоваться.
pub(crate) fn n83995(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41716);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 42677);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 43259);
					match s {
						[] => true,
						['л', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 42678);
			match s {
				[] => true,
				['е', 'г', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'м', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ирализоваться.
pub(crate) fn n83996(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41717);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 42679);
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

/// Words ending -крализоваться.
pub(crate) fn n83997(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41718);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 42680);
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

/// Words ending -орализоваться.
pub(crate) fn n83998(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41719);
	match s {
		[] => true,
		['м', 'е', 'д', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -трализоваться.
pub(crate) fn n83999(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41720);
	match s {
		[] => true,
		['а', 'е', 'т', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['й', 'е', 'н', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['н', 'е', 'ц', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -урализоваться.
pub(crate) fn n84000(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41721);
	match s {
		[] => true,
		['т', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 43260);
					match s {
						[] => true,
						['н', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', s @ ..] => {
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 43794);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 43795);
											match s {
												[] => true,
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
				['к', s @ ..] => {
					super::push(out, 43261);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 43580);
							match s {
								[] => true,
								['р', 'т', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тализоваться.
pub(crate) fn n78219(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39925);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 41722);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 42681);
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
		['е', s @ ..] => {
			super::push(out, 41723);
			match s {
				[] => true,
				['д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уализоваться.
pub(crate) fn n78220(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 41724);
			match s {
				[] => true,
				['и', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 41725);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 42682);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 43262);
							match s {
								[] => true,
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
					super::push(out, 42683);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 43263);
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

/// The word переорганизоваться.
pub(crate) fn n90058(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word перекультивироваться.
pub(crate) fn n90258(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -аспропагандироваться.
pub(crate) fn n90063(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43745);
	match s {
		[] => true,
		['р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -омандироваться.
pub(crate) fn n87153(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 42723);
	match s {
		[] => true,
		['к', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 43590);
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
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

/// Words ending -ендироваться.
pub(crate) fn n78431(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40011);
	match s {
		[] => true,
		['г', 'е', 'л', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['п', 'с', 'у', 'с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ондироваться.
pub(crate) fn n78432(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40012);
	match s {
		[] => true,
		['з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['п', 'с', 'е', 'р', 'р', 'о', 'к', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ф', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ундироваться.
pub(crate) fn n78433(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40013);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 41790);
			match s {
				[] => true,
				['п', 'м', 'о', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 41791);
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
				['з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 41792);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -одироваться.
pub(crate) fn n68599(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36795);
	match s {
		[] => true,
		['й', s @ ..] => {
			super::push(out, 40014);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 40015);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 41793);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 41794);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 41795);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 42724);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 42725);
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
				['с', s @ ..] => {
					super::push(out, 41796);
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
		['м', s @ ..] => {
			super::push(out, 40016);
			match s {
				[] => true,
				['о', 'к', 'к', 'а', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 40017);
			match s {
				[] => true,
				['а', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 40018);
			match s {
				[] => true,
				['а', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['э', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рдироваться.
pub(crate) fn n68600(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36796);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 40019);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 41797);
					match s {
						[] => true,
						['м', 'о', 'б', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 40020);
			match s {
				[] => true,
				['б', 'е', 'д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -удироваться.
pub(crate) fn n68601(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36797);
	match s {
		[] => true,
		['б', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', 'ш', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьдироваться.
pub(crate) fn n68602(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36798);
	match s {
		[] => true,
		['л', 'а', 'с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юдироваться.
pub(crate) fn n68603(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36799);
	match s {
		[] => true,
		['л', 'к', 'к', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word ресинтезироваться.
pub(crate) fn n90064(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -изироваться.
pub(crate) fn n68614(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36808);
	match s {
		[] => true,
		['а', 'х', 'р', 'а', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 40035);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 41809);
					match s {
						[] => true,
						['р', 'а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 40036);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 41810);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 42736);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', 'ш', 'ь', 'л', 'о', 'б', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 41811);
					match s {
						[] => true,
						['т', s @ ..] => {
							match s {
								[] => true,
								['и', 'м', 'и', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['к', 'е', s @ ..] => {
									match s {
										[] => true,
										['л', 'л', 'о', 'к', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['ъ', 'б', 'о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', 'е', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 41812);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 43289);
									match s {
										[] => true,
										['м', 'и', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['а', s @ ..] => {
													super::push(out, 43746);
													match s {
														[] => true,
														['н', s @ ..] => super::t33::n90260(s, out),
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
									super::push(out, 43290);
									match s {
										[] => true,
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
		['г', s @ ..] => {
			super::push(out, 40037);
			match s {
				[] => true,
				['н', 'о', 'т', 'ф', s @ ..] => {
					match s {
						[] => true,
						['и', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'н', 'о', 'м', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', 'л', s @ ..] => {
					match s {
						[] => true,
						['а', 'т', 'а', 'к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 43291);
									match s {
										[] => true,
										['д', 'и', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['а', s @ ..] => {
													super::push(out, 43747);
													match s {
														[] => true,
														['з', s @ ..] => super::t33::n90261(s, out),
														_ => false,
													}
												}
												['е', 'д', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['и', 'б', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['к', 'э', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 43292);
									match s {
										[] => true,
										['и', 'т', 'э', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['е', 'д', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['н', 'и', 'м', 'р', 'е', 'т', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', 'д', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ф', s @ ..] => {
									match s {
										[] => true,
										['и', 'м', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['е', 'д', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', 'о', 'м', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['х', 'и', 'с', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 40038);
			match s {
				[] => true,
				['и', s @ ..] => {
					match s {
						[] => true,
						['л', 'а', 'в', 'н', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'б', 'и', 'г', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', 'о', 'ф', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 40039);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 41813);
					match s {
						[] => true,
						['о', 'р', 'в', 'е', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => super::t33::n78470(s, out),
		['м', s @ ..] => super::t33::n78471(s, out),
		['н', s @ ..] => super::t33::n78472(s, out),
		['о', 'р', 'е', 'г', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['п', 'и', 'т', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['р', s @ ..] => super::t33::n78475(s, out),
		['с', 'э', 'у', 'а', 'д', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', s @ ..] => super::t33::n78477(s, out),
		['у', 'б', 'а', 'т', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ф', 'р', 'о', 'м', 'а', 'т', 'е', 'м', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 40044);
			match s {
				[] => true,
				['а', 'ф', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'т', 'е', 'ф', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word отрецензироваться.
pub(crate) fn n90140(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word отлицензироваться.
pub(crate) fn n90141(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -узироваться.
pub(crate) fn n68617(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36811);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 40049);
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

/// The word отникелироваться.
pub(crate) fn n90142(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -илироваться.
pub(crate) fn n68633(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36826);
	match s {
		[] => true,
		['г', 'и', 'н', 'н', 'а', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['к', 'л', 'а', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['м', 'и', 'с', 'с', 'а', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['п', s @ ..] => {
			match s {
				[] => true,
				['м', 'о', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 42804);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 43347);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['э', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			match s {
				[] => true,
				['а', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'ф', 'с', 'о', 'ф', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', 'к', 'о', 'б', 'р', 'а', 'к', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', s @ ..] => {
			match s {
				[] => true,
				['е', 'ц', 'а', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', 'е', 'в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 42805);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 43348);
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
				['у', 'б', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 40074);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 41851);
					match s {
						[] => true,
						['р', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 43349);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 43350);
									match s {
										[] => true,
										['р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 43351);
									match s {
										[] => true,
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
				['ф', 'а', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ц', 'а', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -олироваться.
pub(crate) fn n68635(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36828);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 40075);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 41852);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 42806);
							match s {
								[] => true,
								['л', 'п', 'е', 'т', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['м', 'а', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 40076);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 41853);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 41854);
					match s {
						[] => true,
						['т', 'о', 'р', 'п', s @ ..] => {
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
		['п', s @ ..] => {
			super::push(out, 40077);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 41855);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 41856);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 42807);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 42808);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 42809);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 43352);
									match s {
										[] => true,
										['с', 'к', 'э', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 41857);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 42810);
							match s {
								[] => true,
								['т', 'н', 'и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 41858);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 42811);
							match s {
								[] => true,
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
					super::push(out, 41859);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 42812);
							match s {
								[] => true,
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
			super::push(out, 40078);
			match s {
				[] => true,
				['е', 'д', 'н', 'а', 'б', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 41860);
					match s {
						[] => true,
						['н', 'о', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 43353);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 43640);
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
		_ => false,
	}
}

/// Words ending -слироваться.
pub(crate) fn n68636(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36829);
	match s {
		[] => true,
		['н', 'а', 'р', 'т', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 41861);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 42813);
							match s {
								[] => true,
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

/// Words ending -улироваться.
pub(crate) fn n68637(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36830);
	match s {
		[] => true,
		['б', 'а', 'т', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 40079);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 41862);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 42814);
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
				['е', 'р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42815);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 42816);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 42817);
							match s {
								[] => true,
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 42818);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 42819);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 42820);
							match s {
								[] => true,
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
		['д', 'о', 'м', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['к', s @ ..] => {
			match s {
				[] => true,
				['и', 'т', 'р', 'а', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ь', 'л', 'а', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 42821);
							match s {
								[] => true,
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
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 41863);
					match s {
						[] => true,
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 41864);
					match s {
						[] => true,
						['о', 'ф', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 43354);
									match s {
										[] => true,
										['р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 43355);
									match s {
										[] => true,
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
				['у', 'к', 'к', 'а', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 42822);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['э', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 40080);
			match s {
				[] => true,
				['а', 'р', 'г', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', 'а', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', 'о', 'к', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['р', 'т', 'а', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 40081);
			match s {
				[] => true,
				['п', 'а', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'р', 'к', 'и', 'м', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 40082);
			match s {
				[] => true,
				['и', 'п', 'а', 'к', 'е', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', 'е', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -флироваться.
pub(crate) fn n68638(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36831);
	match s {
		[] => true,
		['у', 'м', 'а', 'к', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 41865);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 42823);
							match s {
								[] => true,
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

/// Words ending -родекламироваться.
pub(crate) fn n90144(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43766);
	match s {
		[] => true,
		['п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -азрекламироваться.
pub(crate) fn n90145(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43767);
	match s {
		[] => true,
		['р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ммироваться.
pub(crate) fn n68645(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36838);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 40095);
			match s {
				[] => true,
				['р', 'г', 'о', 'р', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42832);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 42833);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 42834);
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
		['у', s @ ..] => {
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 41875);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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
						['о', s @ ..] => {
							super::push(out, 42835);
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

/// Words ending -ромироваться.
pub(crate) fn n78565(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40096);
	match s {
		[] => true,
		['х', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 42836);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 43356);
							match s {
								[] => true,
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

