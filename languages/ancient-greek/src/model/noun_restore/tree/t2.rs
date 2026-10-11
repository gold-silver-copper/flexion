use flexion::Path;

/// Words ending -τροσ.
pub(crate) fn n2021(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1989);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 4032);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 5562);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6295);
							s.is_empty()
						}
						['π', 'π', 'ι', '\u{1}', s @ ..] => {
							super::push(out, 6296);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 5563);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 4033);
			match s {
				[] => true,
				['μ', 'α', 'ι', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 5564);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 4034);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 4035);
			match s {
				[] => true,
				['α', s @ ..] => {
					match s {
						[] => true,
						['β', 'α', 'λ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 6297);
							s.is_empty()
						}
						['λ', 'η', s @ ..] => {
							super::push(out, 6298);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υροσ.
pub(crate) fn n2022(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1990);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 4036);
			match s {
				[] => true,
				['τ', s @ ..] => {
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 6299);
							match s {
								[] => true,
								['ε', 'κ', 'ο', s @ ..] => {
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 6580);
											s.is_empty()
										}
										['υ', s @ ..] => {
											super::push(out, 6581);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 6300);
							match s {
								[] => true,
								['α', 'λ', '\u{1}', s @ ..] => {
									super::push(out, 6512);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 4037);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5565);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 5566);
					match s {
						[] => true,
						['α', s @ ..] => {
							match s {
								[] => true,
								['δ', 'υ', 'ε', 'ψ', '\u{1}', s @ ..] => {
									super::push(out, 6513);
									s.is_empty()
								}
								['θ', s @ ..] => {
									super::push(out, 6514);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 6515);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 4038);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 4039);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 4040);
			match s {
				[] => true,
				['ε', '\u{1}', s @ ..] => {
					super::push(out, 5567);
					s.is_empty()
				}
				['σ', 'ι', s @ ..] => {
					super::push(out, 5568);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 4041);
			match s {
				[] => true,
				['ρ', 'ο', 'μ', '\u{1}', s @ ..] => {
					super::push(out, 5569);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 4042);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 5570);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 5571);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 6301);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 6302);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 5572);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 6303);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 5573);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 5574);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 5575);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 5576);
					match s {
						[] => true,
						['α', 'ρ', 'τ', '\u{1}', s @ ..] => {
							super::push(out, 6304);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 4043);
			match s {
				[] => true,
				['α', 'π', '\u{1}', s @ ..] => {
					super::push(out, 5577);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 4044);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 4045);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5578);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word αφροσ.
pub(crate) fn n5726(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5579);
	s.is_empty()
}

/// The word ταφροσ.
pub(crate) fn n5727(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5580);
	s.is_empty()
}

/// Words ending -εφροσ.
pub(crate) fn n4132(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4047);
	s.is_empty()
}

/// Words ending -ιφροσ.
pub(crate) fn n4133(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4048);
	s.is_empty()
}

/// Words ending -χροσ.
pub(crate) fn n2024(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1992);
	s.is_empty()
}

/// Words ending -ωροσ.
pub(crate) fn n2025(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1993);
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 4049);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 4050);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word τραγωδιογραφοσ.
pub(crate) fn n6791(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6597);
	s.is_empty()
}

/// Words ending -φοριον.
pub(crate) fn n4816(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4702);
	s.is_empty()
}

/// Words ending -χοριον.
pub(crate) fn n4817(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4703);
	match s {
		[] => true,
		['ι', 'κ', '\u{1}', s @ ..] => {
			super::push(out, 5915);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -πριον.
pub(crate) fn n2955(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2895);
	s.is_empty()
}

/// Words ending -ρριον.
pub(crate) fn n2956(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2896);
	s.is_empty()
}

/// Words ending -τριον.
pub(crate) fn n2957(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2897);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 4704);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 4705);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 4706);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υριον.
pub(crate) fn n2958(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2898);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 4707);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 4708);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 4709);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 4710);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 4711);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 5916);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 4712);
			s.is_empty()
		}
		['φ', s @ ..] => {
			super::push(out, 4713);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωριον.
pub(crate) fn n2959(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2899);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 4714);
			s.is_empty()
		}
		_ => false,
	}
}

