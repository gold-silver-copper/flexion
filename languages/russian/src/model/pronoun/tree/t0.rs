use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['е', s @ ..] => {
			super::push(out, 1);
			s.is_empty()
		}
		['й', s @ ..] => {
			super::push(out, 2);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 8);
					match s {
						[] => true,
						['\u{11}', s @ ..] => {
							super::push(out, 14);
							match s {
								[] => true,
								['с', '\u{1}', s @ ..] => {
									super::push(out, 29);
									s.is_empty()
								}
								_ => false,
							}
						}
						['\u{12}', s @ ..] => {
							super::push(out, 15);
							match s {
								[] => true,
								['ч', s @ ..] => {
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 40);
											s.is_empty()
										}
										['и', s @ ..] => {
											super::push(out, 41);
											s.is_empty()
										}
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
					super::push(out, 9);
					match s {
						[] => true,
						['\u{12}', s @ ..] => {
							super::push(out, 16);
							match s {
								[] => true,
								['к', s @ ..] => {
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 42);
											s.is_empty()
										}
										['я', s @ ..] => {
											super::push(out, 43);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['\u{13}', s @ ..] => {
							super::push(out, 17);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 10);
					match s {
						[] => true,
						['\u{11}', s @ ..] => {
							super::push(out, 18);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 30);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 44);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['\u{12}', s @ ..] => {
							super::push(out, 19);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 31);
									s.is_empty()
								}
								['г', s @ ..] => {
									super::push(out, 32);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 33);
									match s {
										[] => true,
										['и', 'н', '\u{1}', s @ ..] => {
											super::push(out, 45);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 34);
									s.is_empty()
								}
								_ => false,
							}
						}
						['\u{13}', s @ ..] => {
							super::push(out, 20);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 35);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 36);
									match s {
										[] => true,
										['а', 'к', 'и', 'н', '-', s @ ..] => {
											super::push(out, 46);
											match s {
												[] => true,
												['й', 'о', 'к', 'а', 'к', '\u{1}', s @ ..] => {
													super::push(out, 49);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 11);
					match s {
						[] => true,
						['\u{12}', s @ ..] => {
							super::push(out, 21);
							s.is_empty()
						}
						['\u{13}', 'р', s @ ..] => {
							super::push(out, 22);
							match s {
								[] => true,
								['о', 'т', 'о', 'к', 'е', s @ ..] => {
									super::push(out, 37);
									s.is_empty()
								}
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
			super::push(out, 3);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 4);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 12);
					match s {
						[] => true,
						['\u{13}', 'и', 'л', '-', 'й', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 38);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 39);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 13);
					match s {
						[] => true,
						['\u{12}', s @ ..] => {
							super::push(out, 23);
							match s {
								[] => true,
								['-', s @ ..] => {
									match s {
										[] => true,
										['й', s @ ..] => {
											super::push(out, 47);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 48);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['\u{13}', s @ ..] => {
							super::push(out, 24);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 5);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['\u{11}', s @ ..] => {
							super::push(out, 25);
							s.is_empty()
						}
						['\u{12}', s @ ..] => {
							super::push(out, 26);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 6);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 7);
			match s {
				[] => true,
				['д', '\u{13}', 'у', 'б', 'и', 'н', '-', 'й', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 27);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 28);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

