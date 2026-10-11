use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['ν', 'η', s @ ..] => {
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 1);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 7);
							s.is_empty()
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 2);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 8);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 27);
									s.is_empty()
								}
								_ => false,
							}
						}
						['η', s @ ..] => {
							super::push(out, 9);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', s @ ..] => super::t0::n4(s, out),
				['λ', s @ ..] => {
					super::push(out, 4);
					match s {
						[] => true,
						['α', 'τ', s @ ..] => {
							super::push(out, 22);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 5);
					match s {
						[] => true,
						['ο', 'κ', 'ε', 'ο', s @ ..] => {
							super::push(out, 23);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 24);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 6);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 25);
							match s {
								[] => true,
								['ρ', 'γ', 'ε', 'ο', s @ ..] => {
									super::push(out, 70);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 26);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 71);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -θην.
pub(crate) fn n4(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 10);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 28);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 29);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 30);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 72);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 129);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 130);
									match s {
										[] => true,
										['τ', 'α', 'κ', 'ο', s @ ..] => {
											super::push(out, 158);
											s.is_empty()
										}
										_ => false,
									}
								}
								['υ', s @ ..] => {
									super::push(out, 131);
									s.is_empty()
								}
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
			super::push(out, 11);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 31);
					s.is_empty()
				}
				['τ', 'ε', 'ο', s @ ..] => {
					super::push(out, 32);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 12);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 33);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 73);
							s.is_empty()
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 34);
					match s {
						[] => true,
						['ο', 'λ', 'υ', s @ ..] => {
							super::push(out, 74);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 35);
					match s {
						[] => true,
						['ο', 'κ', s @ ..] => {
							super::push(out, 75);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 36);
					match s {
						[] => true,
						['ε', 'τ', 'ο', s @ ..] => {
							super::push(out, 76);
							s.is_empty()
						}
						['ω', 'χ', s @ ..] => {
							super::push(out, 77);
							match s {
								[] => true,
								['ε', 'ο', s @ ..] => {
									super::push(out, 132);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 13);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 37);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 38);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 14);
			match s {
				[] => true,
				['ε', 'γ', 'γ', 'η', 'ρ', s @ ..] => {
					super::push(out, 39);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 15);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 40);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 78);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 79);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 133);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 80);
							s.is_empty()
						}
						['φ', s @ ..] => {
							super::push(out, 81);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 41);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 42);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 16);
			match s {
				[] => true,
				['ε', 'κ', s @ ..] => {
					super::push(out, 43);
					s.is_empty()
				}
				['η', 'θ', s @ ..] => {
					super::push(out, 44);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 45);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 17);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 46);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 82);
							match s {
								[] => true,
								['ε', 'π', s @ ..] => {
									super::push(out, 134);
									s.is_empty()
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 83);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 135);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 84);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 136);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 159);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 85);
							match s {
								[] => true,
								['σ', 'ε', 'ι', s @ ..] => {
									super::push(out, 137);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 47);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 86);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 87);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 88);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 138);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 48);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 89);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 90);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 139);
									s.is_empty()
								}
								_ => false,
							}
						}
						['λ', s @ ..] => {
							super::push(out, 91);
							match s {
								[] => true,
								['η', 'ν', s @ ..] => {
									super::push(out, 140);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 92);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 141);
									s.is_empty()
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 93);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 142);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 94);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 143);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 49);
					match s {
						[] => true,
						['μ', 'ω', 'ν', s @ ..] => {
							super::push(out, 95);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 50);
					match s {
						[] => true,
						['α', 'π', s @ ..] => {
							super::push(out, 96);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 97);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 51);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 98);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 18);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 52);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 99);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 53);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 54);
					match s {
						[] => true,
						['η', 'μ', 'ε', 'ο', s @ ..] => {
							super::push(out, 100);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 55);
					match s {
						[] => true,
						['λ', 'ο', s @ ..] => {
							super::push(out, 101);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 56);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => super::t0::n20(s, out),
		['χ', s @ ..] => super::t0::n21(s, out),
		['ω', s @ ..] => {
			super::push(out, 21);
			match s {
				[] => true,
				['ρ', 'υ', s @ ..] => {
					super::push(out, 69);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 128);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φθην.
pub(crate) fn n20(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 57);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 102);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 58);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 103);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 144);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 104);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 145);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 105);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 146);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 160);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 147);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 161);
									match s {
										[] => true,
										['ξ', 'ε', 'ρ', s @ ..] => {
											super::push(out, 164);
											s.is_empty()
										}
										['ο', s @ ..] => {
											super::push(out, 165);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 59);
			match s {
				[] => true,
				['λ', 'ε', s @ ..] => {
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 148);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 149);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 162);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 60);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 106);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 150);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 107);
					match s {
						[] => true,
						['θ', 'ε', 'π', s @ ..] => {
							super::push(out, 151);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 108);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 109);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 61);
			match s {
				[] => true,
				['ε', 'π', 'ε', 'ο', s @ ..] => {
					super::push(out, 110);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 62);
			match s {
				[] => true,
				['λ', 'γ', s @ ..] => {
					super::push(out, 111);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 112);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 113);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χθην.
pub(crate) fn n21(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 63);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 114);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 152);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', 'λ', s @ ..] => {
					super::push(out, 115);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 116);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 117);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 64);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 118);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 153);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 154);
							match s {
								[] => true,
								['ε', 'ι', s @ ..] => {
									super::push(out, 163);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 119);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 65);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 120);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 121);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 122);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 66);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 123);
					match s {
						[] => true,
						['δ', 'ε', 'π', 'υ', s @ ..] => {
							super::push(out, 155);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 156);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 124);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 67);
			match s {
				[] => true,
				['ε', 'δ', s @ ..] => {
					super::push(out, 125);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 68);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 126);
					match s {
						[] => true,
						['ζ', s @ ..] => {
							super::push(out, 157);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ψ', s @ ..] => {
					super::push(out, 127);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

