use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['\u{11}', s @ ..] => {
			super::push(out, 1);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 4);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 5);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 2);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6);
					match s {
						[] => true,
						['т', 'с', 'и', 'р', 'т', '\u{1}', s @ ..] => {
							super::push(out, 19);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'т', 'с', 'е', 'в', 'д', '\u{1}', s @ ..] => {
					super::push(out, 7);
					s.is_empty()
				}
				['к', 'о', 'р', 'о', 'с', '\u{1}', s @ ..] => {
					super::push(out, 8);
					s.is_empty()
				}
				['л', 'о', 'г', 'у', 'г', '\u{1}', s @ ..] => {
					super::push(out, 9);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 10);
					match s {
						[] => true,
						['и', 'д', 'о', '\u{1}', s @ ..] => {
							super::push(out, 20);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 21);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 11);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 12);
					match s {
						[] => true,
						['м', 'е', 'с', 'о', 'в', '\u{1}', s @ ..] => {
							super::push(out, 22);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 23);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 3);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13);
					match s {
						[] => true,
						['р', 'о', 'т', 'л', 'о', 'п', '\u{1}', s @ ..] => {
							super::push(out, 24);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 25);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', 'р', 'ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
					super::push(out, 14);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 15);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 16);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 26);
							match s {
								[] => true,
								['е', s @ ..] => {
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 32);
											s.is_empty()
										}
										['т', 'я', s @ ..] => {
											super::push(out, 33);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', 'с', 'о', 'н', 'я', 'в', 'е', 'д', '\u{1}', s @ ..] => {
							super::push(out, 27);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 17);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 28);
							match s {
								[] => true,
								['с', 'ь', 'м', 'е', 'с', 'о', 'в', '\u{1}', s @ ..] => {
									super::push(out, 30);
									s.is_empty()
								}
								_ => false,
							}
						}
						['я', s @ ..] => {
							super::push(out, 29);
							match s {
								[] => true,
								['с', 'е', 'д', 'ь', s @ ..] => {
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 34);
											match s {
												[] => true,
												['е', 'с', 'о', 'в', '\u{1}', s @ ..] => {
													super::push(out, 40);
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 35);
											s.is_empty()
										}
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
					super::push(out, 18);
					match s {
						[] => true,
						['т', s @ ..] => {
							match s {
								[] => true,
								['а', 'ц', 'д', 'а', 'н', s @ ..] => {
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 36);
											s.is_empty()
										}
										['н', 'и', 'д', 'о', '\u{1}', s @ ..] => {
											super::push(out, 37);
											s.is_empty()
										}
										['р', 'ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
											super::push(out, 38);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 39);
											s.is_empty()
										}
										_ => false,
									}
								}
								['я', s @ ..] => {
									super::push(out, 31);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

