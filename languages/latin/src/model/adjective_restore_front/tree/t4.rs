use flexion::Path;

/// Words ending -mni.
pub(crate) fn n853(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 842);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2962);
			match s {
				[] => true,
				['r', 'c', 'e', 's', 'c', 'i', 'b', 'i', 'l', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 6065);
					s.is_empty()
				}
				['t', 'u', 'r', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 6066);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2963);
			match s {
				[] => true,
				['n', 's', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 6067);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2964);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 2965);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6068);
					match s {
						[] => true,
						['i', 'l', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 8925);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2966);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 6069);
					s.is_empty()
				}
				['n', 'i', 't', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 6070);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6071);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nni.
pub(crate) fn n854(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 843);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2967);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6072);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6073);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 6074);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2968);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2969);
			match s {
				[] => true,
				['s', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 6075);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6076);
					s.is_empty()
				}
				['x', 'u', 's', s @ ..] => {
					super::push(out, 6077);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2970);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 6078);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 6079);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6080);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 6081);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2971);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6082);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 6083);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 6084);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 6085);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6086);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 8926);
							s.is_empty()
						}
						['u', 's', s @ ..] => {
							super::push(out, 8927);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oni.
pub(crate) fn n855(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 844);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 2972);
			match s {
				[] => true,
				['l', 'i', s @ ..] => {
					super::push(out, 6087);
					match s {
						[] => true,
						['t', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 10756);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 10757);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 2973);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 2974);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 2975);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 2976);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6088);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 6089);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 2977);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6090);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 6091);
					match s {
						[] => true,
						['y', 'l', 'l', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 8928);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6092);
					match s {
						[] => true,
						['n', 'a', 't', s @ ..] => {
							super::push(out, 8929);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2978);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2979);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pni.
pub(crate) fn n856(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 845);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2980);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 6093);
					s.is_empty()
				}
				['e', 'n', 'i', 't', 'e', 'n', 's', '\u{1}', s @ ..] => {
					super::push(out, 6094);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8930);
							s.is_empty()
						}
						['a', 't', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 8931);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 's', 'i', 'b', 'i', 'l', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 6095);
					s.is_empty()
				}
				['t', 'i', 'e', 'n', 's', '\u{1}', s @ ..] => {
					super::push(out, 6096);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2981);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 6097);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 8932);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 8933);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6098);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2982);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 2983);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6099);
					match s {
						[] => true,
						['n', 'a', s @ ..] => {
							match s {
								[] => true,
								['n', 's', s @ ..] => {
									super::push(out, 10758);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 10759);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 6100);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 8934);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 8935);
							match s {
								[] => true,
								['u', 's', '\u{1}', s @ ..] => {
									super::push(out, 10760);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6101);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 6102);
					match s {
						[] => true,
						['r', 'a', 'n', 's', s @ ..] => {
							super::push(out, 8936);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 10761);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2984);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 6103);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6104);
					match s {
						[] => true,
						['t', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 10762);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 10763);
									match s {
										[] => true,
										['n', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 11684);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 6105);
					match s {
						[] => true,
						['s', 'i', 'b', 'i', 'l', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 8937);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2985);
			match s {
				[] => true,
				['o', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 8938);
							match s {
								[] => true,
								['p', 't', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 10764);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 8939);
							match s {
								[] => true,
								['i', 's', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 10765);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2986);
			match s {
				[] => true,
				['n', 'i', 't', s @ ..] => {
					super::push(out, 6106);
					match s {
						[] => true,
						['u', 's', '\u{1}', s @ ..] => {
							super::push(out, 8940);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6107);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 8941);
							s.is_empty()
						}
						['u', 's', '\u{1}', s @ ..] => {
							super::push(out, 8942);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -qni.
pub(crate) fn n857(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 846);
	match s {
		[] => true,
		['u', 'i', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6108);
					match s {
						[] => true,
						['t', 'a', s @ ..] => {
							match s {
								[] => true,
								['n', 'd', s @ ..] => {
									super::push(out, 10766);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 10767);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 6109);
					match s {
						[] => true,
						['a', 't', 'i', s @ ..] => {
							super::push(out, 8943);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 10768);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10769);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6110);
					match s {
						[] => true,
						['e', 'n', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 10770);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10771);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 6111);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rni.
pub(crate) fn n858(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 847);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2987);
			match s {
				[] => true,
				['t', 'i', 'o', 'n', 'a', 'b', 'i', 'l', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 6112);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2988);
			match s {
				[] => true,
				['g', 'i', 'b', 'i', 'l', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 6113);
					s.is_empty()
				}
				['l', 'i', 'g', 'i', 'o', 's', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 6114);
					s.is_empty()
				}
				['m', 'e', 'd', 'i', 'a', 'b', 'i', 'l', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 6115);
					s.is_empty()
				}
				['p', s @ ..] => {
					match s {
						[] => true,
						['a', 'r', 'a', 'b', 'i', 'l', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 8944);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 8945);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 8946);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 8947);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 6116);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2989);
			match s {
				[] => true,
				['d', 'e', 'n', s @ ..] => {
					match s {
						[] => true,
						['d', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 8948);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 8949);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 6117);
					s.is_empty()
				}
				['s', 'u', 'r', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 6118);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6119);
					match s {
						[] => true,
						['u', 's', '\u{1}', s @ ..] => {
							super::push(out, 8950);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2990);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 2991);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sni.
pub(crate) fn n859(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 848);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2992);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 6120);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6121);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 8951);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 8952);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 6122);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6123);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 2993);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 6124);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6125);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 8953);
							match s {
								[] => true,
								['b', s @ ..] => {
									match s {
										[] => true,
										['e', 'n', s @ ..] => {
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 12099);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 12100);
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', s @ ..] => {
											super::push(out, 11685);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', 't', 'u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11686);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11687);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 8954);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 6126);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2994);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 6127);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 6128);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6129);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 8955);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 8956);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 6130);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8957);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 8958);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6131);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2995);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 6132);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 8959);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 8960);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 6133);
					match s {
						[] => true,
						['n', 'i', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', 'i', 'm', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2996);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 6134);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 6135);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 2997);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6136);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 8961);
							s.is_empty()
						}
						['r', 'a', s @ ..] => {
							super::push(out, 8962);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 10772);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 10773);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6137);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 8963);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 8964);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['n', 's', s @ ..] => {
											super::push(out, 11688);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 11689);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2998);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6138);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 8965);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 6139);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 6140);
					match s {
						[] => true,
						['g', 'a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 11690);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11691);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 10774);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 8966);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 8967);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 8968);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6141);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 8969);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 8970);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 8971);
							match s {
								[] => true,
								['c', 't', s @ ..] => {
									match s {
										[] => true,
										['i', 's', s @ ..] => {
											super::push(out, 11692);
											s.is_empty()
										}
										['u', 'r', s @ ..] => {
											super::push(out, 11693);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 10775);
									s.is_empty()
								}
								['m', 'e', 'n', 't', 'a', 'l', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 10776);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2999);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6142);
					s.is_empty()
				}
				['e', s @ ..] => super::t4::n6570(s, out),
				['f', s @ ..] => {
					super::push(out, 6144);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 6145);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 6146);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6147);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 6148);
					s.is_empty()
				}
				['t', s @ ..] => super::t4::n6576(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tni.
pub(crate) fn n860(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 849);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3000);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 3001);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 6150);
					s.is_empty()
				}
				['g', 'e', 'r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'i', 'm', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 6151);
					match s {
						[] => true,
						['l', 'e', 'c', 't', 'u', 'a', s @ ..] => {
							super::push(out, 8975);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 6152);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 8976);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 8977);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6153);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 8978);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 8979);
							match s {
								[] => true,
								['m', 'p', s @ ..] => {
									super::push(out, 10781);
									match s {
										[] => true,
										['t', 'u', s @ ..] => {
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 12101);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 12102);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 8980);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 10782);
									match s {
										[] => true,
										['n', s @ ..] => {
											match s {
												[] => true,
												['c', s @ ..] => {
													super::push(out, 12103);
													match s {
														[] => true,
														['t', 'u', s @ ..] => super::t4::n14258(s, out),
														_ => false,
													}
												}
												['g', s @ ..] => {
													super::push(out, 12104);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 8981);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 10783);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 11694);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 10784);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 8982);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 10785);
									match s {
										[] => true,
										['t', 'i', 'o', 'n', 'a', 'l', 'i', 's', '\u{1}', s @ ..] => {
											super::push(out, 11695);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', 's', s @ ..] => {
									super::push(out, 10786);
									match s {
										[] => true,
										['c', 'e', 'n', s @ ..] => {
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 12105);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 12106);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 8983);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 10787);
									match s {
										[] => true,
										['n', s @ ..] => {
											match s {
												[] => true,
												['c', 't', 'u', 'r', s @ ..] => {
													super::push(out, 12107);
													s.is_empty()
												}
												['g', s @ ..] => {
													super::push(out, 12108);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 8984);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3002);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 6154);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 8985);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 8986);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3003);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 6155);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6156);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 8987);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 8988);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 8989);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3004);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6157);
					match s {
						[] => true,
						['c', 't', s @ ..] => {
							match s {
								[] => true,
								['a', 't', s @ ..] => {
									super::push(out, 10788);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 10789);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 8990);
							s.is_empty()
						}
						['n', 's', 'i', s @ ..] => {
							super::push(out, 8991);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 8992);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 6158);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 6159);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 8993);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 8994);
							match s {
								[] => true,
								['a', s @ ..] => super::t4::n12267(s, out),
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 8995);
							s.is_empty()
						}
						['t', 'u', s @ ..] => super::t4::n9975(s, out),
						_ => false,
					}
				}
				['o', s @ ..] => super::t4::n6589(s, out),
				_ => false,
			}
		}
		['u', s @ ..] => super::t4::n3099(s, out),
		_ => false,
	}
}

/// Words ending -uni.
pub(crate) fn n861(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 850);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 3006);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 3007);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 6161);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 6162);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 6163);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6164);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 6165);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 6166);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6167);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3008);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vni.
pub(crate) fn n862(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 851);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3009);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 6168);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 6169);
					s.is_empty()
				}
				['s', 'u', 'r', s @ ..] => {
					super::push(out, 6170);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3010);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 6171);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6172);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 8999);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 6173);
					match s {
						[] => true,
						['t', 'i', 'g', 'a', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 10795);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 10796);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 6174);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3011);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 6175);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 6176);
					match s {
						[] => true,
						['i', 'o', 's', 'i', s @ ..] => {
							super::push(out, 9000);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 10797);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10798);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 6177);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 6178);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 6179);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 9001);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 9002);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 9003);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10799);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10800);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 6180);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9004);
							match s {
								[] => true,
								['t', 'o', s @ ..] => {
									super::push(out, 10801);
									match s {
										[] => true,
										['r', 'i', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 11698);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 9005);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3012);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word sumissilarebil.
pub(crate) fn n14083(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -utangil.
pub(crate) fn n10083(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 10839);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 10840);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ugil.
pub(crate) fn n3214(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3105);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 6301);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 6302);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9080);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 10841);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 10842);
							match s {
								[] => true,
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 10843);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', 'u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['s', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 6303);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lil.
pub(crate) fn n923(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 912);
	match s {
		[] => true,
		['a', 'c', 'e', s @ ..] => {
			super::push(out, 3106);
			match s {
				[] => true,
				['u', 's', '\u{1}', s @ ..] => {
					super::push(out, 6304);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mil.
pub(crate) fn n924(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 913);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3107);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 6305);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 9081);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 9082);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 6306);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10844);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10845);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 3108);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3109);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 3110);
			match s {
				[] => true,
				['e', 't', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 6307);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 3111);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 3112);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nil.
pub(crate) fn n925(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 914);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 3113);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 3114);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6308);
					match s {
						[] => true,
						['r', 'i', 'f', s @ ..] => {
							super::push(out, 9083);
							match s {
								[] => true,
								['o', 'l', 'i', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 10846);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 9084);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11707);
											s.is_empty()
										}
										['s', '\u{1}', s @ ..] => {
											super::push(out, 11708);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 6309);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 6310);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6311);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 3115);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3116);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6312);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 6313);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6314);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10847);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10848);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['q', s @ ..] => {
			super::push(out, 3117);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 3118);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pil.
pub(crate) fn n926(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 915);
	match s {
		[] => true,
		['s', 'i', 'e', 'n', 's', 'i', 's', '\u{1}', s @ ..] => {
			super::push(out, 3119);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -qil.
pub(crate) fn n927(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 916);
	match s {
		[] => true,
		['u', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6315);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 9085);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 9086);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 6316);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 9087);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 9088);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 9089);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6317);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 9090);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 9091);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -til.
pub(crate) fn n929(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 918);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3120);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 6318);
					s.is_empty()
				}
				['t', 'u', 'r', s @ ..] => {
					super::push(out, 6319);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3121);
			match s {
				[] => true,
				['r', 'a', s @ ..] => {
					match s {
						[] => true,
						['r', 'i', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 9092);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 10849);
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									super::push(out, 10850);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 3122);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3123);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 3124);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 3125);
			match s {
				[] => true,
				['e', 'r', 'a', 't', s @ ..] => {
					super::push(out, 6320);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 9093);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 9094);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3126);
			match s {
				[] => true,
				['r', 'g', s @ ..] => {
					super::push(out, 6321);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vil.
pub(crate) fn n930(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 919);
	match s {
		[] => true,
		['e', 'r', s @ ..] => {
			super::push(out, 3127);
			match s {
				[] => true,
				['p', 'o', 'l', 'i', 't', 'a', 'n', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 6322);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -xil.
pub(crate) fn n931(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 920);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3128);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3129);
			match s {
				[] => true,
				['v', 'i', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 6323);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -dnaroilem.
pub(crate) fn n14088(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12111);
	match s {
		[] => true,
		['u', 's', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -snaroilem.
pub(crate) fn n14089(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12112);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -utaroilem.
pub(crate) fn n14090(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['r', 'u', 's', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['s', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -narbmem.
pub(crate) fn n6874(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9149);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 9150);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 9151);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -omem.
pub(crate) fn n3325(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3206);
	match s {
		[] => true,
		['r', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', s @ ..] => {
					match s {
						[] => true,
						['b', 'i', 'l', 'i', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['d', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 'o', 'r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nem.
pub(crate) fn n972(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 961);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 3207);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6418);
					match s {
						[] => true,
						['c', 'i', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 10874);
									s.is_empty()
								}
								['o', 'r', s @ ..] => {
									super::push(out, 10875);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10876);
									s.is_empty()
								}
								_ => false,
							}
						}
						['x', s @ ..] => {
							super::push(out, 9152);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 6419);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 6420);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['n', 's', s @ ..] => {
											super::push(out, 11726);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 11727);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 10877);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 6421);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 10878);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 10879);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3208);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 3209);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6422);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6423);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6424);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9153);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 10880);
									match s {
										[] => true,
										['n', 's', s @ ..] => {
											super::push(out, 11728);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 11729);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 10881);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 9154);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3210);
			match s {
				[] => true,
				['i', 't', s @ ..] => {
					super::push(out, 6425);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10882);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10883);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rem.
pub(crate) fn n973(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 962);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3211);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 9155);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 9156);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 3212);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6426);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 9157);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 9158);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 10884);
									s.is_empty()
								}
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11730);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11731);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 6427);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6428);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3213);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 3214);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3215);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 6429);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6430);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 3216);
			match s {
				[] => true,
				['a', 't', s @ ..] => {
					super::push(out, 6431);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10885);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10886);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3217);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tsem.
pub(crate) fn n3337(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3218);
	match s {
		[] => true,
		['u', 's', '\u{1}', s @ ..] => {
			super::push(out, 6432);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tem.
pub(crate) fn n975(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 964);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3219);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 6433);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6434);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 6435);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 6436);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6437);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10887);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10888);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3220);
			s.is_empty()
		}
		['h', s @ ..] => {
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 6438);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 6439);
					match s {
						[] => true,
						['s', 't', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 9159);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3221);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 6440);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 6441);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 6442);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6443);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 10889);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 10890);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3222);
			match s {
				[] => true,
				['p', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 6444);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 6445);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 6446);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3223);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6447);
					s.is_empty()
				}
				['t', 'u', 'r', s @ ..] => {
					super::push(out, 6448);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -xem.
pub(crate) fn n976(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 965);
	match s {
		[] => true,
		['i', 'c', 'a', 'n', s @ ..] => {
			match s {
				[] => true,
				['e', 'n', 's', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 6449);
					s.is_empty()
				}
				['u', 's', '\u{1}', s @ ..] => {
					super::push(out, 6450);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word silibaresim.
pub(crate) fn n14091(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -utiresim.
pub(crate) fn n10210(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['r', 'u', 's', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['s', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tim.
pub(crate) fn n984(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 973);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 3240);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3241);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', 'i', 'm', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3242);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 3243);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xim.
pub(crate) fn n985(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 974);
	s.is_empty()
}

/// Words ending -itum.
pub(crate) fn n3411(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3287);
	s.is_empty()
}

/// Words ending -ttum.
pub(crate) fn n3412(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3288);
	s.is_empty()
}

/// Words ending -utum.
pub(crate) fn n3413(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3289);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6506);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 9187);
					s.is_empty()
				}
				['t', 'u', 'r', s @ ..] => {
					super::push(out, 9188);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 6507);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rep.
pub(crate) fn n1158(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1146);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3552);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 6740);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 9337);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 9338);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 3553);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 3554);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6741);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 6742);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 9339);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 9340);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 11756);
											s.is_empty()
										}
										['u', 's', s @ ..] => {
											super::push(out, 11757);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 9341);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6743);
					match s {
						[] => true,
						['d', 'e', 'n', 'd', s @ ..] => {
							super::push(out, 9342);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 9343);
							s.is_empty()
						}
						['s', 'u', 'r', s @ ..] => {
							super::push(out, 9344);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 6744);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 9345);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['n', 's', s @ ..] => {
											super::push(out, 11758);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 11759);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 9346);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 10963);
									match s {
										[] => true,
										['n', 's', s @ ..] => {
											super::push(out, 11760);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 11761);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 10964);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6745);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6746);
					match s {
						[] => true,
						['r', 'i', s @ ..] => {
							super::push(out, 9347);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 3555);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6747);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 9348);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 10965);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 10966);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 10967);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 9349);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 9350);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6748);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 6749);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 9351);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 9352);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 9353);
							match s {
								[] => true,
								['a', 'n', 's', '\u{1}', s @ ..] => {
									super::push(out, 10968);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 6750);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['e', 'n', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 11762);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11763);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 10969);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 9354);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['n', 's', s @ ..] => {
											super::push(out, 11764);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 11765);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3556);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 6751);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 6752);
					match s {
						[] => true,
						['r', 'i', 'n', 'a', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 10970);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 10971);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 10972);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 6753);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 6754);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6755);
					s.is_empty()
				}
				['q', s @ ..] => {
					super::push(out, 6756);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6757);
					s.is_empty()
				}
				['s', 'u', 'r', s @ ..] => {
					super::push(out, 6758);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 6759);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 6760);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 3557);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6761);
					s.is_empty()
				}
				['i', s @ ..] => super::t4::n7291(s, out),
				['l', s @ ..] => {
					super::push(out, 6763);
					match s {
						[] => true,
						['a', 'g', s @ ..] => {
							super::push(out, 9359);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => super::t4::n7293(s, out),
				['r', s @ ..] => super::t4::n7294(s, out),
				['u', s @ ..] => super::t4::n7295(s, out),
				_ => false,
			}
		}
		['g', s @ ..] => super::t4::n3705(s, out),
		['h', s @ ..] => super::t4::n3706(s, out),
		['i', s @ ..] => super::t4::n3707(s, out),
		['l', s @ ..] => super::t4::n3708(s, out),
		['m', s @ ..] => super::t4::n3709(s, out),
		['n', s @ ..] => super::t4::n3710(s, out),
		['o', s @ ..] => super::t4::n3711(s, out),
		['p', s @ ..] => super::t4::n3712(s, out),
		['q', s @ ..] => super::t4::n3713(s, out),
		['r', s @ ..] => super::t4::n3714(s, out),
		['s', s @ ..] => super::t4::n3715(s, out),
		['t', s @ ..] => super::t4::n3716(s, out),
		['u', s @ ..] => super::t4::n3717(s, out),
		['v', s @ ..] => super::t4::n3718(s, out),
		_ => false,
	}
}

/// Words ending -tsop.
pub(crate) fn n3793(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 6885);
			match s {
				[] => true,
				['l', 'e', 'n', 'o', 'i', 'd', 'a', 'l', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 9477);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 6886);
			match s {
				[] => true,
				['o', s @ ..] => {
					match s {
						[] => true,
						['n', 'e', 'n', 'd', s @ ..] => {
							super::push(out, 11020);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 11021);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 6887);
			match s {
				[] => true,
				['e', 'm', s @ ..] => {
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 11022);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 11023);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -top.
pub(crate) fn n1192(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1180);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3641);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 6888);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 6889);
					s.is_empty()
				}
				['t', 'u', 'r', s @ ..] => {
					super::push(out, 6890);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3642);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3643);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6891);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 6892);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 9478);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['n', 's', s @ ..] => {
											super::push(out, 11823);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 11824);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 9479);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 6893);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6894);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 9480);
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 11024);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11025);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3644);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 3645);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -earp.
pub(crate) fn n3800(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3647);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 6895);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 6896);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9481);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 9482);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 11026);
							match s {
								[] => true,
								['e', 'n', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 12121);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 12122);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 9483);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 11027);
							match s {
								[] => true,
								['e', 'n', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 12123);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 12124);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', 'c', s @ ..] => {
							super::push(out, 11028);
							match s {
								[] => true,
								['t', 'u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 12125);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 12126);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 11029);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 9484);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['m', 'a', 't', s @ ..] => {
									super::push(out, 11825);
									match s {
										[] => true,
										['u', s @ ..] => {
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 12234);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 12235);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', 'i', s @ ..] => {
									super::push(out, 11826);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 12127);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 12128);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 11030);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 9485);
					match s {
						[] => true,
						['g', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 11827);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 11828);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 11031);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 9486);
					match s {
						[] => true,
						['r', 'a', s @ ..] => {
							super::push(out, 11032);
							match s {
								[] => true,
								['n', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 12129);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 12130);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 12131);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 12132);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 6897);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9487);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 11033);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 11034);
							s.is_empty()
						}
						['t', 'o', s @ ..] => {
							super::push(out, 11035);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 9488);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 11036);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 11037);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 9489);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 11829);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 11830);
									match s {
										[] => true,
										['n', s @ ..] => {
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 12236);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 12237);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 11038);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 11039);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 11040);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 9490);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 9491);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 11831);
									match s {
										[] => true,
										['n', s @ ..] => {
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 12238);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 12239);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 11832);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 6898);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 9492);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 9493);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => super::t4::n7441(s, out),
		['g', s @ ..] => {
			super::push(out, 6900);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9497);
					match s {
						[] => true,
						['v', 'i', 's', 'u', 'r', s @ ..] => super::t4::n12620(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t4::n7443(s, out),
		['l', s @ ..] => super::t4::n7444(s, out),
		['m', s @ ..] => super::t4::n7445(s, out),
		['n', s @ ..] => super::t4::n7446(s, out),
		['o', s @ ..] => super::t5::n7447(s, out),
		['p', s @ ..] => super::t5::n7448(s, out),
		['r', s @ ..] => super::t5::n7449(s, out),
		['s', s @ ..] => super::t5::n7450(s, out),
		['t', s @ ..] => super::t5::n7451(s, out),
		['u', s @ ..] => super::t5::n7452(s, out),
		['v', s @ ..] => super::t5::n7453(s, out),
		_ => false,
	}
}

/// Words ending -erp.
pub(crate) fn n1194(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1182);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 3652);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 3653);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['b', 'u', 'n', 'd', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 9533);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 9534);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 9535);
							match s {
								[] => true,
								['u', 's', s @ ..] => {
									super::push(out, 11067);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 3654);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6914);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 11068);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11069);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 11845);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 11846);
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 12157);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 12158);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6915);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 3655);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 3656);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 3657);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 3658);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 6916);
					match s {
						[] => true,
						['n', 's', '\u{1}', s @ ..] => {
							super::push(out, 9536);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3659);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9537);
							s.is_empty()
						}
						['o', 's', 'i', s @ ..] => {
							super::push(out, 9538);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 11070);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11071);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -irp.
pub(crate) fn n1195(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1183);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3660);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 3661);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6917);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 6918);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 6919);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 9539);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 11072);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 11073);
									match s {
										[] => true,
										['s', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 6920);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3662);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 3663);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 3664);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 6921);
					match s {
						[] => true,
						['a', 't', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 9540);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 3665);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 9541);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 9542);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 11074);
									s.is_empty()
								}
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11847);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11848);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -orp.
pub(crate) fn n1196(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1184);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 3666);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6922);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 9543);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 9544);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 11075);
									s.is_empty()
								}
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11849);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11850);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6923);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 6924);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 6925);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 6926);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 3667);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6927);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 9545);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 9546);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 9547);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 6928);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 9548);
							match s {
								[] => true,
								['e', 'n', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 11851);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11852);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 9549);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 9550);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 9551);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6929);
					match s {
						[] => true,
						['d', 'e', 'n', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 11076);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11077);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 9552);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 9553);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 6930);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9554);
							match s {
								[] => true,
								['m', 'a', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 12159);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 12160);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 'u', s @ ..] => {
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 12161);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 12162);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 9555);
							match s {
								[] => true,
								['n', 'a', s @ ..] => {
									match s {
										[] => true,
										['n', 'd', s @ ..] => {
											super::push(out, 11853);
											s.is_empty()
										}
										['t', 'u', 'r', s @ ..] => {
											super::push(out, 11854);
											s.is_empty()
										}
										_ => false,
									}
								}
								['v', s @ ..] => {
									super::push(out, 11078);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 6931);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 11079);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11080);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 6932);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9556);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 9557);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 6933);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 9558);
							s.is_empty()
						}
						['d', 'e', 'n', 'd', s @ ..] => {
							super::push(out, 9559);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 9560);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 9561);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 9562);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 11081);
									match s {
										[] => true,
										['n', 's', s @ ..] => {
											super::push(out, 11855);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 11856);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 11082);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11083);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'u', 'r', s @ ..] => {
							super::push(out, 9563);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 3668);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 6934);
					match s {
						[] => true,
						['c', 't', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 11084);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11085);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 6935);
					match s {
						[] => true,
						['c', 'e', s @ ..] => super::t5::n10681(s, out),
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 6936);
					s.is_empty()
				}
				['u', s @ ..] => super::t5::n7482(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t5::n3822(s, out),
		['f', s @ ..] => super::t5::n3823(s, out),
		['g', s @ ..] => super::t5::n3824(s, out),
		['h', s @ ..] => super::t5::n3825(s, out),
		['i', s @ ..] => super::t5::n3826(s, out),
		['l', s @ ..] => super::t5::n3827(s, out),
		['m', s @ ..] => super::t5::n3828(s, out),
		['n', s @ ..] => super::t5::n3829(s, out),
		['p', s @ ..] => super::t5::n3830(s, out),
		['r', s @ ..] => super::t5::n3831(s, out),
		['s', s @ ..] => super::t5::n3832(s, out),
		['t', s @ ..] => super::t5::n3833(s, out),
		['v', s @ ..] => super::t5::n3834(s, out),
		['x', s @ ..] => super::t5::n3835(s, out),
		_ => false,
	}
}

/// Words ending -ertup.
pub(crate) fn n7552(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 9625);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 9626);
			match s {
				[] => true,
				['c', 'e', 'n', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 11886);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 11887);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word sudnesnecer.
pub(crate) fn n14167(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word snesnecer.
pub(crate) fn n14168(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -nilcer.
pub(crate) fn n10800(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 12174);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 12175);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 11893);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 11147);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ulcer.
pub(crate) fn n7597(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7034);
	match s {
		[] => true,
		['d', 'e', 'n', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 11148);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 11149);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', 'u', s @ ..] => {
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 11150);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 11151);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ocer.
pub(crate) fn n3916(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3759);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 7035);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 9656);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 9657);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 11152);
							s.is_empty()
						}
						['o', 's', 'c', 'e', 'n', 'd', s @ ..] => {
							super::push(out, 11153);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 7036);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 7037);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 7038);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rcer.
pub(crate) fn n3917(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3760);
	match s {
		[] => true,
		['e', 'd', s @ ..] => {
			super::push(out, 7039);
			match s {
				[] => true,
				['u', 't', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 9658);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ucer.
pub(crate) fn n3919(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3762);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 7040);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 7041);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 7042);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9659);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 9660);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -der.
pub(crate) fn n1233(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1221);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3763);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 7043);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 7044);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 7045);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 3764);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 3765);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 7046);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 7047);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 3766);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3767);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 3768);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 7048);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 9661);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 9662);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 9663);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 7049);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -fer.
pub(crate) fn n1235(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1223);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3769);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 3770);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3771);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 7050);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 7051);
					match s {
						[] => true,
						['u', 'r', 'a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 11894);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11895);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11896);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11897);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 3772);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7052);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 7053);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 7054);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3773);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 7055);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 7056);
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 11154);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 11155);
									match s {
										[] => true,
										['d', 'a', s @ ..] => {
											match s {
												[] => true,
												['n', s @ ..] => {
													super::push(out, 12176);
													s.is_empty()
												}
												['t', 'u', 'r', s @ ..] => {
													super::push(out, 12177);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 'u', 'r', s @ ..] => {
					super::push(out, 7057);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 7058);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3774);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7059);
					match s {
						[] => true,
						['c', 't', 'u', 's', s @ ..] => {
							super::push(out, 9664);
							s.is_empty()
						}
						['g', 'a', s @ ..] => {
							match s {
								[] => true,
								['n', 's', s @ ..] => {
									super::push(out, 11156);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 11157);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 7060);
					match s {
						[] => true,
						['n', 'a', s @ ..] => {
							match s {
								[] => true,
								['n', 's', s @ ..] => {
									super::push(out, 11158);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 11159);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 7061);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 9665);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 9666);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 9667);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3775);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 7062);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 7063);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 7064);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 9668);
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 11160);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11161);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 7065);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ger.
pub(crate) fn n1236(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1224);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3776);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 3777);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3778);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 7066);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 7067);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 7068);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 9669);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 9670);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 7069);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 7070);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 3779);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 3780);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 9671);
							s.is_empty()
						}
						['t', 'u', s @ ..] => {
							super::push(out, 9672);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 11162);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11163);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 3781);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 3782);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 7071);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 7072);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 3783);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ier.
pub(crate) fn n1237(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1225);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 3784);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 7073);
					s.is_empty()
				}
				['v', 'i', 't', 'a', s @ ..] => {
					match s {
						[] => true,
						['n', 'd', s @ ..] => {
							super::push(out, 9673);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 9674);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ler.
pub(crate) fn n1238(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1226);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3785);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 7074);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 7075);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 7076);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 7077);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 9675);
							match s {
								[] => true,
								['v', s @ ..] => {
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 11898);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 11899);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 9676);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 11164);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11165);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['x', s @ ..] => {
					super::push(out, 7078);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3786);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 7079);
					s.is_empty()
				}
				['g', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9677);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 9678);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 7080);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3787);
			match s {
				[] => true,
				['g', 'i', s @ ..] => {
					super::push(out, 7081);
					match s {
						[] => true,
						['o', 's', 'i', 's', s @ ..] => {
							super::push(out, 9679);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 3788);
			match s {
				[] => true,
				['i', 'g', 'i', 'o', 's', 'i', s @ ..] => {
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 9680);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 9681);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3789);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 3790);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mer.
pub(crate) fn n1239(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1227);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3791);
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9682);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 9683);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 9684);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3792);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7082);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 9685);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 9686);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 7083);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 7084);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 9687);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 9688);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 7085);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 11166);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 11167);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3793);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 7086);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9689);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 9690);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 7087);
					match s {
						[] => true,
						['i', 's', 'c', 'e', 'n', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 11168);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11169);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 7088);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 7089);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 7090);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 7091);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 7092);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3794);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 7093);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 7094);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 7095);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 7096);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 9691);
							s.is_empty()
						}
						['u', 's', s @ ..] => {
							super::push(out, 9692);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3795);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 7097);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 7098);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 7099);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ner.
pub(crate) fn n1240(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1228);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3796);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 7100);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 7101);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 7102);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 11170);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11171);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 3797);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 3798);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3799);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 7103);
					match s {
						[] => true,
						['o', 'r', 'm', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 9693);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'u', 's', s @ ..] => {
					super::push(out, 7104);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 3800);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 3801);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 3802);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 3803);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7105);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 7106);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -per.
pub(crate) fn n1241(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1229);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3804);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 7107);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 7108);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3805);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 7109);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 9694);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 3806);
			match s {
				[] => true,
				['e', 't', s @ ..] => {
					super::push(out, 7110);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 11172);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11173);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 7111);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3807);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 7112);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 7113);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 7114);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 3808);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3809);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7115);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 7116);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 9695);
							match s {
								[] => true,
								['e', 'n', 's', 'i', s @ ..] => {
									super::push(out, 11174);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 9696);
							match s {
								[] => true,
								['s', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 11175);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 7117);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 9697);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 9698);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 9699);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3810);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 9700);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 9701);
							s.is_empty()
						}
						['t', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 11176);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11177);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 7118);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3811);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 7119);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 7120);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 7121);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 7122);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -qer.
pub(crate) fn n1242(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1230);
	match s {
		[] => true,
		['u', 'i', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7123);
					match s {
						[] => true,
						['s', 'c', 'e', 'n', 'd', s @ ..] => {
							super::push(out, 9702);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 9703);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 'n', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 9704);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 9705);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 7124);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ser.
pub(crate) fn n1243(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1231);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3812);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 7125);
					match s {
						[] => true,
						['u', 't', 'a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 11900);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11901);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11902);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11903);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 7126);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 3813);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 7127);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 9706);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 11178);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11179);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 9707);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 7128);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['b', 'e', 'n', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 11904);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11905);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', 't', 'u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11906);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11907);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3814);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3815);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 7129);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 7130);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 7131);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3816);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 3817);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7132);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 7133);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 9708);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 9709);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['n', 's', s @ ..] => {
											super::push(out, 11908);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 11909);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 7134);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 11180);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11181);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 11910);
											s.is_empty()
										}
										['u', 's', s @ ..] => {
											super::push(out, 11911);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 7135);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 9710);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 9711);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3818);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7136);
					s.is_empty()
				}
				['i', 'n', s @ ..] => {
					super::push(out, 7137);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 9712);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 9713);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3819);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 7138);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 7139);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 7140);
					match s {
						[] => true,
						['i', 'n', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 12178);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 12179);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 'u', s @ ..] => {
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 12180);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 12181);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 11182);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 7141);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 7142);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ter.
pub(crate) fn n1244(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1232);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3820);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 7143);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 7144);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3821);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 7145);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 7146);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 9714);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 9715);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 9716);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 7147);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 7148);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3822);
			match s {
				[] => true,
				['c', 'u', s @ ..] => {
					super::push(out, 7149);
					match s {
						[] => true,
						['l', 'a', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 11183);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 11184);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 7150);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3823);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3824);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7151);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 7152);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 7153);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 9717);
							match s {
								[] => true,
								['o', 'l', 'a', 'r', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 11185);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 7154);
					match s {
						[] => true,
						['d', 'e', 'n', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 11186);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11187);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 11188);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11189);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3825);
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 9718);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 9719);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uer.
pub(crate) fn n1245(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1233);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 3826);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 9720);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 9721);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 3827);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ver.
pub(crate) fn n1246(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1234);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 3828);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 7155);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9722);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 9723);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 7156);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3829);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 7157);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 7158);
					s.is_empty()
				}
				['n', 'c', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 9724);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 9725);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 9726);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 7159);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 7160);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 9727);
							s.is_empty()
						}
						['u', 's', s @ ..] => {
							super::push(out, 9728);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 7161);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3830);
			match s {
				[] => true,
				['c', 'a', 't', s @ ..] => {
					super::push(out, 7162);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 9729);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 9730);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 7163);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -dnacirbur.
pub(crate) fn n14177(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12182);
	s.is_empty()
}

/// Words ending -snacirbur.
pub(crate) fn n14178(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12183);
	s.is_empty()
}

/// Words ending -utacirbur.
pub(crate) fn n14179(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 12250);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 12251);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vomes.
pub(crate) fn n7872(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7282);
	s.is_empty()
}

/// Words ending -pmes.
pub(crate) fn n4114(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3942);
	s.is_empty()
}

/// Words ending -umes.
pub(crate) fn n4115(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3943);
	s.is_empty()
}

/// Words ending -nes.
pub(crate) fn n1311(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1299);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3944);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 7283);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 7284);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3945);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 7285);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3946);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 3947);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 3948);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', 't', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', 'l', 'i', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['r', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3949);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oes.
pub(crate) fn n1312(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1300);
	s.is_empty()
}

/// Words ending -pes.
pub(crate) fn n1313(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1301);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3950);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							match s {
								[] => true,
								['b', 'i', 'l', 'i', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['d', 'u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['s', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'u', s @ ..] => {
									match s {
										[] => true,
										['r', 'u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['s', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 3951);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 3952);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 3953);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 7286);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 7287);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3954);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7288);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 9778);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 9779);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 11221);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 11222);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 7289);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 9780);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 11223);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 11224);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 7290);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9781);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 9782);
							s.is_empty()
						}
						['r', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 9783);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 9784);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 3955);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -qes.
pub(crate) fn n1314(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1302);
	match s {
		[] => true,
		['u', 'u', s @ ..] => {
			super::push(out, 3956);
			match s {
				[] => true,
				['t', 'u', 'r', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 7291);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -res.
pub(crate) fn n1315(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1303);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3957);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 7292);
					s.is_empty()
				}
				['t', 'u', 'r', s @ ..] => {
					super::push(out, 7293);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 3958);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 3959);
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 9785);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 9786);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 9787);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3960);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7294);
					match s {
						[] => true,
						['t', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 9788);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 3961);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 3962);
			match s {
				[] => true,
				['s', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 7295);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 7296);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 3963);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 3964);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 9789);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['i', 'f', 'o', 'l', 'i', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 11225);
									s.is_empty()
								}
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11916);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11917);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 3965);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 3966);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 3967);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7297);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 9790);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 9791);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 9792);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 7298);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 9793);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 11226);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 11227);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ses.
pub(crate) fn n1316(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1304);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3968);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 3969);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7299);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 7300);
					s.is_empty()
				}
				_ => false,
			}
		}
		['q', s @ ..] => {
			super::push(out, 3970);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 3971);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tes.
pub(crate) fn n1317(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1305);
	match s {
		[] => true,
		['i', 'f', s @ ..] => {
			super::push(out, 3972);
			match s {
				[] => true,
				['e', 'r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', 's', '\u{1}', s @ ..] => {
							super::push(out, 9794);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ues.
pub(crate) fn n1318(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1306);
	s.is_empty()
}

/// Words ending -ves.
pub(crate) fn n1319(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1307);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 3973);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 9795);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 9796);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 3974);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xes.
pub(crate) fn n1320(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1308);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 3975);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 7301);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 7302);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 3976);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 3977);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -llehcyes.
pub(crate) fn n1321(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'r', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 3978);
			s.is_empty()
		}
		['e', 'n', 's', 'i', 's', '\u{1}', s @ ..] => {
			super::push(out, 3979);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uts.
pub(crate) fn n1358(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1344);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 4063);
			match s {
				[] => true,
				['i', 'o', 's', 'i', 's', s @ ..] => {
					super::push(out, 7397);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 4064);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 4065);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 9841);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 9842);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -dnecudbus.
pub(crate) fn n14180(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12184);
	s.is_empty()
}

/// Words ending -snecudbus.
pub(crate) fn n14181(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12185);
	s.is_empty()
}

/// Words ending -fbus.
pub(crate) fn n4248(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4071);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 7407);
			match s {
				[] => true,
				['c', 'i', 'e', 'n', s @ ..] => {
					match s {
						[] => true,
						['d', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 11251);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 11252);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 9851);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ibus.
pub(crate) fn n4250(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4073);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 7408);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 7409);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 9852);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 9853);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 9854);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 11253);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 11921);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 11922);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 12186);
											match s {
												[] => true,
												['u', 'c', s @ ..] => {
													match s {
														[] => true,
														['e', 'n', s @ ..] => super::t5::n14304(s, out),
														['t', 'u', s @ ..] => super::t5::n14305(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['i', s @ ..] => {
											super::push(out, 12187);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 9855);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 7410);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 7411);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 7412);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 9856);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 9857);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 11254);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 11255);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lbus.
pub(crate) fn n4251(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4074);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7413);
			match s {
				[] => true,
				['t', 'u', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 11256);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 11257);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 7414);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 9858);
					match s {
						[] => true,
						['t', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 11923);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11924);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 7415);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 9859);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 9860);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 11258);
							match s {
								[] => true,
								['n', 's', s @ ..] => {
									super::push(out, 11925);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 11926);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['n', 'a', 'l', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 11927);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 11928);
									s.is_empty()
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 12188);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 12189);
											match s {
												[] => true,
												['i', 'm', 'u', 's', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 11259);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 7416);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 9861);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 9862);
					s.is_empty()
				}
				['s', 't', 'r', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 9863);
					s.is_empty()
				}
				['t', 'u', 'r', s @ ..] => {
					super::push(out, 9864);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mbus.
pub(crate) fn n4252(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4075);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 7417);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 7418);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 7419);
			match s {
				[] => true,
				['t', 'u', 'r', s @ ..] => {
					super::push(out, 9865);
					match s {
						[] => true,
						['u', 's', '\u{1}', s @ ..] => {
							super::push(out, 11260);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 7420);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 9866);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 9867);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', 's', s @ ..] => {
									super::push(out, 11929);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 11930);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nbus.
pub(crate) fn n4253(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4076);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7421);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 7422);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 7423);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 7424);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -obus.
pub(crate) fn n4254(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4077);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 7425);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 7426);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 7427);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 7428);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 7429);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pbus.
pub(crate) fn n4255(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4078);
	match s {
		[] => true,
		['o', 'n', s @ ..] => {
			super::push(out, 7430);
			match s {
				[] => true,
				['e', 'n', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 11261);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 11262);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rbus.
pub(crate) fn n4256(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4079);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 7431);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 9868);
					match s {
						[] => true,
						['t', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 11263);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 't', 'u', 's', s @ ..] => {
					super::push(out, 9869);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 7432);
			match s {
				[] => true,
				['d', 'e', 'n', 'd', s @ ..] => {
					super::push(out, 9870);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 9871);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 9872);
					s.is_empty()
				}
				['s', 'u', 'r', s @ ..] => {
					super::push(out, 9873);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 7433);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 7434);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 9874);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sbus.
pub(crate) fn n4257(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4080);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 7435);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 9875);
					match s {
						[] => true,
						['e', 'n', 'd', s @ ..] => {
							super::push(out, 11264);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 7436);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 9876);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 9877);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 7437);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tbus.
pub(crate) fn n4258(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4081);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 7438);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 7439);
			match s {
				[] => true,
				['l', 'i', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 11265);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11931);
									match s {
										[] => true,
										['i', 'm', 'u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ubus.
pub(crate) fn n4259(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4082);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 7440);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 7441);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 9878);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 9879);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 7442);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -cus.
pub(crate) fn n1362(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1348);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 4084);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7443);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 9880);
							match s {
								[] => true,
								['e', 'n', 's', s @ ..] => {
									super::push(out, 11266);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 11267);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11268);
									match s {
										[] => true,
										['u', 's', s @ ..] => {
											super::push(out, 11932);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 9881);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 7444);
					match s {
						[] => true,
						['d', 'e', 'n', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 11269);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11270);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 11271);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 11272);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 11273);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 11274);
									match s {
										[] => true,
										['s', '\u{1}', s @ ..] => {
											super::push(out, 11933);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 9882);
							match s {
								[] => true,
								['u', 's', s @ ..] => {
									super::push(out, 11275);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 7445);
					match s {
						[] => true,
						['a', 'm', 'a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 11934);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11935);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11936);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11937);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 7446);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 7447);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', 'u', 'v', 'e', 'n', 's', 'i', 's', '\u{1}', s @ ..] => {
			super::push(out, 4085);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 4086);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 4087);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 4088);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 4089);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -dus.
pub(crate) fn n1363(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1349);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4090);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 7448);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 7449);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 9883);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'i', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 7450);
					s.is_empty()
				}
				['t', 'u', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 9884);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 9885);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', 't', 'i', 'c', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 4091);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 4092);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 4093);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -eus.
pub(crate) fn n1364(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1350);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 4094);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 4095);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 4096);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 4097);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 4098);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -fus.
pub(crate) fn n1365(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1351);
	match s {
		[] => true,
		['f', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7451);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 7452);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 9886);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 9887);
							s.is_empty()
						}
						['g', 'e', 'n', 'd', s @ ..] => {
							super::push(out, 9888);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 9889);
							s.is_empty()
						}
						['x', 'u', 'r', s @ ..] => {
							super::push(out, 9890);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 7453);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 11276);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 11277);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 11278);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 11279);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 7454);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 9891);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['n', 's', s @ ..] => {
											super::push(out, 11938);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 11939);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 9892);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 9893);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 7455);
					match s {
						[] => true,
						['a', 'g', 'a', s @ ..] => {
							match s {
								[] => true,
								['n', 'd', s @ ..] => {
									super::push(out, 11280);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 11281);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 9894);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 7456);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 9895);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 9896);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 9897);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['n', 's', s @ ..] => {
											super::push(out, 11940);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 11941);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', 'u', 'r', s @ ..] => {
							super::push(out, 9898);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gus.
pub(crate) fn n1366(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1352);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 4099);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 4100);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 4101);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ius.
pub(crate) fn n1367(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1353);
	s.is_empty()
}

/// Words ending -lus.
pub(crate) fn n1368(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1354);
	match s {
		[] => true,
		['c', 'a', 't', s @ ..] => {
			super::push(out, 4102);
			match s {
				[] => true,
				['u', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 9899);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 9900);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mus.
pub(crate) fn n1369(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1355);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 4103);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 7457);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 7458);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 4104);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 7459);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 7460);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 9901);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11942);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11943);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 7461);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 9902);
							match s {
								[] => true,
								['a', 'n', 's', '\u{1}', s @ ..] => {
									super::push(out, 11282);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 4105);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pus.
pub(crate) fn n1370(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1356);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 4106);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 9903);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 9904);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 9905);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 9906);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 11283);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 11284);
									s.is_empty()
								}
								['x', s @ ..] => {
									super::push(out, 11285);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 9907);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 9908);
							match s {
								[] => true,
								['a', 'v', s @ ..] => {
									super::push(out, 11286);
									match s {
										[] => true,
										['i', 's', 'u', s @ ..] => {
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 12190);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 12191);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 11287);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 9909);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 9910);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 9911);
							match s {
								[] => true,
								['a', 't', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 11944);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 11945);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 9912);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 11288);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 11289);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 9913);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 9914);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 11290);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 11291);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 11946);
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 11947);
											match s {
												[] => true,
												['i', 'n', 'a', s @ ..] => {
													match s {
														[] => true,
														['n', 's', s @ ..] => super::t5::n14290(s, out),
														['t', s @ ..] => super::t5::n14291(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['s', s @ ..] => {
											super::push(out, 11948);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 11292);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 9915);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4107);
			match s {
				[] => true,
				['n', 'a', s @ ..] => {
					super::push(out, 7462);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 4108);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7463);
					match s {
						[] => true,
						['d', 'e', s @ ..] => {
							super::push(out, 9916);
							match s {
								[] => true,
								['n', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 11949);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11950);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 7464);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 7465);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 9917);
							match s {
								[] => true,
								['e', 'n', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 11951);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11952);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 4109);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7466);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 9918);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 9919);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 7467);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rus.
pub(crate) fn n1371(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1357);
	match s {
		[] => true,
		['r', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7468);
					match s {
						[] => true,
						['c', 't', 'u', 'r', s @ ..] => {
							super::push(out, 9920);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 9921);
							s.is_empty()
						}
						['p', s @ ..] => {
							match s {
								[] => true,
								['e', 'n', 'd', s @ ..] => {
									super::push(out, 11293);
									s.is_empty()
								}
								['t', 'u', 's', s @ ..] => {
									super::push(out, 11294);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 7469);
					match s {
						[] => true,
						['d', 'e', 'n', 'd', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 9922);
							s.is_empty()
						}
						['s', 'u', 'r', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 9923);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 7470);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sus.
pub(crate) fn n1372(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1358);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 4110);
			match s {
				[] => true,
				['e', 'n', s @ ..] => {
					super::push(out, 7471);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 4111);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7472);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 9924);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 11295);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11296);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 11953);
											s.is_empty()
										}
										['u', s @ ..] => {
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 12192);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 12193);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 7473);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 9925);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 11297);
									s.is_empty()
								}
								['i', 'o', s @ ..] => {
									super::push(out, 11298);
									match s {
										[] => true,
										['s', s @ ..] => {
											match s {
												[] => true,
												['i', s @ ..] => {
													super::push(out, 12194);
													s.is_empty()
												}
												['u', s @ ..] => {
													super::push(out, 12195);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 9926);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['n', 's', s @ ..] => {
											super::push(out, 11954);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 11955);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 4112);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 4113);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tus.
pub(crate) fn n1373(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1359);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 4114);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 4115);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 4116);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 4117);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 4118);
			match s {
				[] => true,
				['r', 'a', s @ ..] => {
					super::push(out, 7474);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -snart.
pub(crate) fn n8156(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7531);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9953);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 11315);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 11316);
					match s {
						[] => true,
						['t', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 12196);
									s.is_empty()
								}
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 12254);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 12255);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 9954);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 9955);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 11317);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 11318);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 11958);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 11959);
							match s {
								[] => true,
								['m', 'a', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 12264);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 12265);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 'u', s @ ..] => {
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 12266);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 12267);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 11960);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 11319);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 9956);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 9957);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 11320);
					match s {
						[] => true,
						['t', 'a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 12197);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 12198);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 9958);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 11961);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 12199);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 12200);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 11962);
							match s {
								[] => true,
								['i', 'g', 'a', s @ ..] => {
									match s {
										[] => true,
										['n', 's', s @ ..] => {
											super::push(out, 12256);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 12257);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 9959);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 11321);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 12201);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 12202);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 11322);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 9960);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 9961);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 9962);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 11323);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 11324);
					match s {
						[] => true,
						['e', 'n', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 12203);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 12204);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', 't', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 12205);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 12206);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 9963);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -irt.
pub(crate) fn n1421(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1407);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4205);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 4206);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7537);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 9966);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 9967);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 11963);
											s.is_empty()
										}
										['t', 'u', 's', s @ ..] => {
											super::push(out, 11964);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 9968);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 9969);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 11325);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 11326);
									s.is_empty()
								}
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 11965);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11966);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 4207);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7538);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 9970);
							match s {
								[] => true,
								['n', 'i', s @ ..] => {
									super::push(out, 11327);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 9971);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 7539);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 4208);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7540);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 7541);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4209);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 4210);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 4211);
			match s {
				[] => true,
				['e', 's', s @ ..] => {
					super::push(out, 7542);
					match s {
						[] => true,
						['i', 'm', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 9972);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 4212);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 4213);
			match s {
				[] => true,
				['a', 'c', 'u', 'l', 'a', 't', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 7543);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 7544);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 9973);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 9974);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 7545);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 4214);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 4215);
			match s {
				[] => true,
				['u', 'n', s @ ..] => {
					super::push(out, 7546);
					match s {
						[] => true,
						['c', 't', 'a', 't', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 9975);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 4216);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7547);
					s.is_empty()
				}
				['p', 'i', 'n', 'o', 's', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 7548);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 7549);
					match s {
						[] => true,
						['a', 'n', s @ ..] => {
							super::push(out, 9976);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 11328);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 9977);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['s', 'i', 'm', 'u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 7550);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 4217);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 7551);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 9978);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 12207);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 12208);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 11967);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 11329);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 9979);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4218);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 4219);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gort.
pub(crate) fn n4406(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4221);
	match s {
		[] => true,
		['l', 'o', 'd', 'y', 't', 'i', 's', '\u{1}', s @ ..] => {
			super::push(out, 7552);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -iort.
pub(crate) fn n4407(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4222);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7553);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 7554);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -urt.
pub(crate) fn n1423(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1409);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 4223);
			match s {
				[] => true,
				['i', 'd', s @ ..] => {
					super::push(out, 7555);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', 's', s @ ..] => {
									super::push(out, 11330);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 11331);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 7556);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 4224);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 4225);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 4226);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nrev.
pub(crate) fn n4518(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4323);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7644);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 10026);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 10027);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 7645);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 7646);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rrev.
pub(crate) fn n4520(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4325);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 7647);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 7648);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -srev.
pub(crate) fn n4521(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4326);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7649);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 10028);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 10029);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 11349);
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 11970);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11971);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 7650);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 7651);
			match s {
				[] => true,
				['t', 'i', s @ ..] => {
					super::push(out, 10030);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 11350);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 11351);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -trev.
pub(crate) fn n4522(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4327);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 7652);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 7653);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 10031);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -surev.
pub(crate) fn n8302(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7654);
	s.is_empty()
}

/// Words ending -vrev.
pub(crate) fn n4524(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4329);
	s.is_empty()
}

/// Words ending -sev.
pub(crate) fn n1484(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1470);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4330);
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 10032);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 10033);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 4331);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 7655);
					s.is_empty()
				}
				['u', 'r', s @ ..] => {
					super::push(out, 7656);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4332);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 4333);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 4334);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 10034);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 10035);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['n', 's', s @ ..] => {
											super::push(out, 11972);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 11973);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 10036);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4335);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tev.
pub(crate) fn n1485(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1471);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 4336);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10037);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 11352);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 11353);
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 11974);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 11975);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 10038);
							match s {
								[] => true,
								['n', 'a', s @ ..] => {
									super::push(out, 11354);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 4337);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -xev.
pub(crate) fn n1486(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1472);
	match s {
		[] => true,
		['a', 't', s @ ..] => {
			super::push(out, 4338);
			match s {
				[] => true,
				['u', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 10039);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 10040);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -aniv.
pub(crate) fn n4549(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4353);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 7676);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 7677);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 7678);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -cniv.
pub(crate) fn n4550(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4354);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 7679);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 7680);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 7681);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -dniv.
pub(crate) fn n4551(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4355);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 7682);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 10056);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 10057);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 7683);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -eniv.
pub(crate) fn n4552(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4356);
	match s {
		[] => true,
		['a', 't', s @ ..] => {
			super::push(out, 7684);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -iniv.
pub(crate) fn n4553(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4357);
	s.is_empty()
}

/// Words ending -nniv.
pub(crate) fn n4554(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4358);
	s.is_empty()
}

/// Words ending -oniv.
pub(crate) fn n4555(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4359);
	match s {
		[] => true,
		['s', 'i', s @ ..] => {
			super::push(out, 7685);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word sutneluniv.
pub(crate) fn n4556(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4360);
	s.is_empty()
}

/// Words ending -oiv.
pub(crate) fn n1496(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1482);
	match s {
		[] => true,
		['l', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 10058);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 10059);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 10060);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 7686);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -piv.
pub(crate) fn n1497(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1483);
	s.is_empty()
}

/// Words ending -riv.
pub(crate) fn n1498(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1484);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 4361);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 7687);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 7688);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 4362);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 4363);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 7689);
					match s {
						[] => true,
						['i', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', 'i', 'm', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 7690);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 11357);
									s.is_empty()
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 11976);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 11977);
											match s {
												[] => true,
												['i', 'm', 'u', 's', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 4364);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 4365);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -siv.
pub(crate) fn n1499(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1485);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 4366);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 4367);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 4368);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 4369);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tiv.
pub(crate) fn n1500(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1486);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4370);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 7691);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 7692);
					s.is_empty()
				}
				['t', 'u', 'r', s @ ..] => {
					super::push(out, 7693);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 4371);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 7694);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 7695);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4372);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7696);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 10061);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 10062);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 7697);
					match s {
						[] => true,
						['o', 'l', 'i', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 10063);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 7698);
					match s {
						[] => true,
						['s', 'i', 's', s @ ..] => {
							super::push(out, 10064);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 4373);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 4374);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 4375);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -viv.
pub(crate) fn n1501(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1487);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 4376);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 7699);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 7700);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4377);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word sudnareficov.
pub(crate) fn n14211(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word snareficov.
pub(crate) fn n14212(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word surutareficov.
pub(crate) fn n14213(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word sutareficov.
pub(crate) fn n14214(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -rutaroballoc.
pub(crate) fn n14300(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12260);
	s.is_empty()
}

/// Words ending -sutaroballoc.
pub(crate) fn n14301(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12261);
	s.is_empty()
}

/// Words ending -rutommoc.
pub(crate) fn n13146(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11446);
	s.is_empty()
}

/// Words ending -sutommoc.
pub(crate) fn n13147(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11447);
	s.is_empty()
}

/// Words ending -ummoc.
pub(crate) fn n5337(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5054);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 8110);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 8111);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 8112);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 11448);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 11449);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 11450);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 12007);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 12008);
									match s {
										[] => true,
										['i', 'c', 'u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 8113);
			s.is_empty()
		}
		['t', 'a', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 10278);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 11451);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 11452);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 10279);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -omoc.
pub(crate) fn n2173(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2127);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 5055);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 5056);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pmoc.
pub(crate) fn n2174(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2128);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5057);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 8114);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 10280);
							match s {
								[] => true,
								['s', 'c', 'e', 'n', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 12009);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 12010);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 10281);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 11453);
									s.is_empty()
								}
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 12011);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 12012);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 8115);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							match s {
								[] => true,
								['b', 'i', 'l', 'i', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['d', 'u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['s', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'u', s @ ..] => {
									match s {
										[] => true,
										['r', 'u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['s', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', 'n', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 10282);
							match s {
								[] => true,
								['t', 'u', 'r', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									match s {
										[] => true,
										['c', 'e', 'p', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', 'n', s @ ..] => {
											match s {
												[] => true,
												['d', 'u', 's', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['s', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', 'u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['s', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 10283);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 10284);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 8116);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5058);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 8117);
					match s {
						[] => true,
						['i', 's', 'c', 'e', 'n', 'd', s @ ..] => {
							super::push(out, 10285);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 10286);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 8118);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 8119);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 8120);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 10287);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 10288);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 8121);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 8122);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5059);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 8123);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', 's', s @ ..] => {
									super::push(out, 11454);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 11455);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 8124);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 8125);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 5060);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8126);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 10289);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 11456);
									match s {
										[] => true,
										['n', s @ ..] => {
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 12210);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 12211);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 'u', s @ ..] => super::t5::n13950(s, out),
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 11457);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => super::t5::n8889(s, out),
				['i', s @ ..] => super::t5::n8890(s, out),
				['o', s @ ..] => super::t5::n8891(s, out),
				['u', s @ ..] => super::t5::n8892(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => super::t5::n5344(s, out),
		['r', s @ ..] => super::t5::n5345(s, out),
		['t', s @ ..] => super::t5::n5346(s, out),
		['u', s @ ..] => super::t5::n5347(s, out),
		_ => false,
	}
}

/// Words ending -dnarbelecnoc.
pub(crate) fn n14229(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12214);
	s.is_empty()
}

/// Words ending -snarbelecnoc.
pub(crate) fn n14230(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12215);
	s.is_empty()
}

/// Words ending -rutarbelecnoc.
pub(crate) fn n14231(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12216);
	s.is_empty()
}

/// Words ending -sutarbelecnoc.
pub(crate) fn n14232(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12217);
	s.is_empty()
}

/// Words ending -rcnoc.
pub(crate) fn n5358(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5073);
	match s {
		[] => true,
		['e', 'd', s @ ..] => {
			super::push(out, 8154);
			match s {
				[] => true,
				['e', 'n', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 11472);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 11473);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 't', 'u', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 11474);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 11475);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ucnoc.
pub(crate) fn n5359(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5074);
	match s {
		[] => true,
		['p', s @ ..] => {
			super::push(out, 8155);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 11476);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 11477);
							match s {
								[] => true,
								['c', 'e', 'n', 's', s @ ..] => {
									super::push(out, 12019);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 11478);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 8156);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -dnoc.
pub(crate) fn n2178(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2132);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 5075);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 8157);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 8158);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 8159);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 10306);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 11479);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 11480);
									match s {
										[] => true,
										['n', s @ ..] => {
											match s {
												[] => true,
												['d', 'u', 's', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['s', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 'u', s @ ..] => {
											match s {
												[] => true,
												['r', 'u', 's', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['s', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 11481);
									match s {
										[] => true,
										['s', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 8160);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5076);
			match s {
				[] => true,
				['c', 'e', s @ ..] => {
					super::push(out, 8161);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 11482);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11483);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 'u', 'r', s @ ..] => {
					super::push(out, 8162);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5077);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 8163);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 8164);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', 's', s @ ..] => {
									super::push(out, 11484);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 11485);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 8165);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5078);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 8166);
					match s {
						[] => true,
						['e', 'n', 'd', s @ ..] => {
							super::push(out, 10307);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 10308);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 10309);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 8167);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 8168);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -enoc.
pub(crate) fn n2179(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2133);
	s.is_empty()
}

/// Words ending -fnoc.
pub(crate) fn n2180(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2134);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5079);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 8169);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 8170);
					match s {
						[] => true,
						['t', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 10310);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5080);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 5081);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 8171);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 8172);
					match s {
						[] => true,
						['e', 'n', s @ ..] => {
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 11486);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 11487);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 8173);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 10311);
							match s {
								[] => true,
								['n', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 12020);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 12021);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 10312);
							match s {
								[] => true,
								['r', 'a', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 12022);
											s.is_empty()
										}
										['t', 'u', 'r', s @ ..] => {
											super::push(out, 12023);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 8174);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 8175);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 10313);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 10314);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 11488);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11489);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['x', s @ ..] => {
					super::push(out, 8176);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 11490);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11491);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 5082);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8177);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 8178);
					match s {
						[] => true,
						['c', 't', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 12218);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 12219);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 12024);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 12025);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 12026);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 11492);
									s.is_empty()
								}
								['e', 'n', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 12027);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 12028);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5083);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 8179);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 8180);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 8181);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 10315);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 11493);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 11494);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 10316);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 8182);
					s.is_empty()
				}
				['t', 'u', 'r', s @ ..] => {
					super::push(out, 8183);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 8184);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5084);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8185);
					match s {
						[] => true,
						['c', 't', 'u', 'r', s @ ..] => {
							super::push(out, 10317);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 8186);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5085);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 8187);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 8188);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 8189);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['b', 'i', 'l', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 11495);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 11496);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11497);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 11498);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11499);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10318);
							match s {
								[] => true,
								['n', 's', s @ ..] => {
									super::push(out, 11500);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 11501);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 10319);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gnoc.
pub(crate) fn n2181(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2135);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5086);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 8190);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 8191);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5087);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 5088);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8192);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 8193);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 10320);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 10321);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 8194);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5089);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 5090);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8195);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 8196);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 8197);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 5091);
			match s {
				[] => true,
				['r', 'a', s @ ..] => {
					match s {
						[] => true,
						['n', 's', s @ ..] => {
							super::push(out, 10322);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 10323);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -inoc.
pub(crate) fn n2182(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2136);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 5092);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 8198);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 8199);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5093);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 5094);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 5095);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 5096);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 5097);
			match s {
				[] => true,
				['u', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 10324);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 10325);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 5098);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 5099);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 8200);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 8201);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 10326);
							match s {
								[] => true,
								['t', 'u', 'r', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 11502);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 10327);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'a', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 11503);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11504);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 10328);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 5100);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lnoc.
pub(crate) fn n2183(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2137);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5101);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 10329);
							match s {
								[] => true,
								['n', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 12029);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 12030);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 10330);
							match s {
								[] => true,
								['r', 'a', 't', 'u', 'r', s @ ..] => {
									super::push(out, 11505);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 8202);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 8203);
					match s {
						[] => true,
						['s', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 11506);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11507);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['i', 'v', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 10331);
							s.is_empty()
						}
						['u', 'r', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 10332);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5102);
			match s {
				[] => true,
				['c', 't', 'u', 'r', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 8204);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5103);
			match s {
				[] => true,
				['d', 'e', 'n', 'd', s @ ..] => {
					super::push(out, 8205);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 8206);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 8207);
					match s {
						[] => true,
						['u', 's', '\u{1}', s @ ..] => {
							super::push(out, 10333);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5104);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 5105);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 8208);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 10334);
							match s {
								[] => true,
								['n', 's', '\u{1}', s @ ..] => {
									super::push(out, 11508);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 8209);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mnoc.
pub(crate) fn n2184(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2138);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5106);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 5107);
			match s {
				[] => true,
				['n', 's', 'u', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 10335);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 10336);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'i', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 10337);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 10338);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5108);
			match s {
				[] => true,
				['t', 'u', 'r', s @ ..] => {
					super::push(out, 8210);
					match s {
						[] => true,
						['u', 's', '\u{1}', s @ ..] => {
							super::push(out, 10339);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nnoc.
pub(crate) fn n2185(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2139);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 5109);
			match s {
				[] => true,
				['t', 'e', 'n', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 10340);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 10341);
							s.is_empty()
						}
						_ => false,
					}
				}
				['x', 'u', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 10342);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 10343);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5110);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word suedionoc.
pub(crate) fn n2186(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2140);
	s.is_empty()
}

/// Words ending -pnoc.
pub(crate) fn n2187(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2141);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5111);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 8211);
					match s {
						[] => true,
						['t', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 11509);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 11510);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							match s {
								[] => true,
								['b', 'i', 'l', 'i', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['d', 'u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['s', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'u', s @ ..] => {
									match s {
										[] => true,
										['r', 'u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['s', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', 'n', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 10344);
							match s {
								[] => true,
								['t', 'u', 'r', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 11511);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									match s {
										[] => true,
										['c', 'e', 'p', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', 'n', s @ ..] => {
											match s {
												[] => true,
												['d', 'u', 's', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['s', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', 'u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['s', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5112);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 5113);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 8212);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', 's', s @ ..] => {
									super::push(out, 11512);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 11513);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 8213);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 5114);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8214);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 10345);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 11514);
									match s {
										[] => true,
										['n', s @ ..] => {
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 12220);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 12221);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 'u', s @ ..] => {
											match s {
												[] => true,
												['r', s @ ..] => {
													super::push(out, 12222);
													s.is_empty()
												}
												['s', '\u{1}', s @ ..] => {
													super::push(out, 12223);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 11515);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 8215);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 8216);
					match s {
						[] => true,
						['d', 'e', 'n', 'd', s @ ..] => {
							super::push(out, 10346);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5115);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', 't', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5116);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 5117);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 8217);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 10347);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 10348);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -qnoc.
pub(crate) fn n2188(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2142);
	match s {
		[] => true,
		['u', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8218);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 8219);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 8220);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 10349);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['e', 'n', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 12031);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 12032);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 11516);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'i', 't', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 11517);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11518);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rnoc.
pub(crate) fn n2189(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2143);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 5118);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 8221);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 8222);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5119);
			match s {
				[] => true,
				['d', 'e', 'n', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 10350);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 10351);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5120);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 8223);
					s.is_empty()
				}
				['d', 'e', 'n', 'd', s @ ..] => {
					super::push(out, 8224);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 8225);
					s.is_empty()
				}
				['s', 'u', 'r', s @ ..] => {
					super::push(out, 8226);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -snoc.
pub(crate) fn n2190(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2144);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5121);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 8227);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 8228);
					match s {
						[] => true,
						['u', 't', 'a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 11519);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 11520);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 8229);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 12033);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 12034);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 11521);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 10352);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 10353);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 8230);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 8231);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 5122);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 8232);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 10354);
							s.is_empty()
						}
						['n', 's', s @ ..] => {
							super::push(out, 10355);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 12035);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 12036);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 8233);
					match s {
						[] => true,
						['s', 'c', s @ ..] => {
							super::push(out, 10356);
							match s {
								[] => true,
								['e', 'n', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 12037);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 12038);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 8234);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 10357);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 10358);
							match s {
								[] => true,
								['b', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 12039);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 12040);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									super::push(out, 11522);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5123);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 8235);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 8236);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', 's', s @ ..] => {
									super::push(out, 11523);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 11524);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 8237);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 10359);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 11525);
									s.is_empty()
								}
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 12041);
											match s {
												[] => true,
												['u', 's', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', '\u{1}', s @ ..] => {
											super::push(out, 12042);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 10360);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 8238);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 10361);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 10362);
							s.is_empty()
						}
						['t', 'u', 'r', s @ ..] => {
							super::push(out, 10363);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 10364);
							s.is_empty()
						}
						_ => false,
					}
				}
				['q', s @ ..] => {
					super::push(out, 8239);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 8240);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5124);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 8241);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 11526);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 12043);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 12044);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 11527);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 8242);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 8243);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 8244);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 8245);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 8246);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5125);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 8247);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10365);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 11528);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 12045);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 12046);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 11529);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 10366);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 8248);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 8249);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 8250);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => super::t5::n5412(s, out),
		['t', s @ ..] => super::t5::n5413(s, out),
		['u', s @ ..] => super::t5::n5414(s, out),
		_ => false,
	}
}

/// Words ending -tnoc.
pub(crate) fn n2191(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2145);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5129);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 8260);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 10377);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 10378);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 11536);
									match s {
										[] => true,
										['d', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 12052);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', 't', 'u', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 10379);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 10380);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 8261);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 8262);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5130);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 8263);
					match s {
						[] => true,
						['t', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 11537);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11538);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 8264);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 8265);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 10381);
							match s {
								[] => true,
								['b', 'r', 'a', s @ ..] => {
									match s {
										[] => true,
										['n', 'd', s @ ..] => {
											super::push(out, 12053);
											s.is_empty()
										}
										['t', 'u', 'r', s @ ..] => {
											super::push(out, 12054);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 10382);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5131);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 8266);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 8267);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 8268);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 8269);
					match s {
						[] => true,
						['n', 'a', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 11539);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 11540);
									s.is_empty()
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 12055);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 12056);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 11541);
									s.is_empty()
								}
								['t', 'u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 12057);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 12058);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5132);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 5133);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 8270);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 10383);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 10384);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 10385);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 10386);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 10387);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 10388);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 8271);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 8272);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 10389);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 10390);
							match s {
								[] => true,
								['t', 'a', s @ ..] => {
									match s {
										[] => true,
										['n', 's', s @ ..] => {
											super::push(out, 12059);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 12060);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 10391);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 8273);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 8274);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 10392);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 10393);
							match s {
								[] => true,
								['e', 'n', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 12061);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 12062);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5134);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 8275);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 8276);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 8277);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10394);
							s.is_empty()
						}
						['e', 'l', 'i', 'o', 's', 'i', s @ ..] => {
							super::push(out, 10395);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 10396);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 10397);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 12063);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 12064);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 8278);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 8279);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', 's', s @ ..] => {
									super::push(out, 11542);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 11543);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -unoc.
pub(crate) fn n2192(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2146);
	s.is_empty()
}

/// Words ending -vnoc.
pub(crate) fn n2193(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2147);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5135);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 5136);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 8280);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 8281);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 10398);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 10399);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5137);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 10400);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 10401);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 8282);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 8283);
					match s {
						[] => true,
						['a', 'n', 's', s @ ..] => {
							super::push(out, 10402);
							s.is_empty()
						}
						['e', 'n', 's', s @ ..] => {
							super::push(out, 10403);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 10404);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5138);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 5139);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -itsed.
pub(crate) fn n5626(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5307);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 8412);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 8413);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 8414);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rtsed.
pub(crate) fn n5627(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5308);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 8415);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 8416);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 10465);
					match s {
						[] => true,
						['t', 'u', 's', s @ ..] => {
							super::push(out, 11588);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 10466);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -used.
pub(crate) fn n2347(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2295);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 5309);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 5310);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 5311);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 5312);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -irapsid.
pub(crate) fn n11893(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['l', 'i', 's', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['t', 'u', s @ ..] => {
			match s {
				[] => true,
				['r', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -itrapsid.
pub(crate) fn n11894(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', 'n', s @ ..] => {
			match s {
				[] => true,
				['d', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'u', s @ ..] => {
			match s {
				[] => true,
				['r', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -epsid.
pub(crate) fn n5715(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5391);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 8456);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 8457);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -opsid.
pub(crate) fn n5717(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5393);
	match s {
		[] => true,
		['n', s @ ..] => {
			match s {
				[] => true,
				['e', 'n', 'd', s @ ..] => {
					super::push(out, 10501);
					s.is_empty()
				}
				['i', 'b', 'i', 'l', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 10502);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 8458);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -upsid.
pub(crate) fn n5718(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5394);
	s.is_empty()
}

/// Words ending -qsid.
pub(crate) fn n2404(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2348);
	match s {
		[] => true,
		['u', 'i', 'r', 'e', 'n', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 8459);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 8460);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ssid.
pub(crate) fn n2405(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2349);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 5395);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 8461);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 8462);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 10503);
							match s {
								[] => true,
								['e', 'r', 'g', 'e', 'n', 's', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 11592);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 10504);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['s', 'u', 'r', s @ ..] => {
							super::push(out, 10505);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 10506);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 8463);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 8464);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5396);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5397);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 5398);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 8465);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 11593);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11594);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tsid.
pub(crate) fn n2406(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2350);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 5399);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 8466);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 8467);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5400);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 5401);
			match s {
				[] => true,
				['g', 'm', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 8468);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 8469);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 10507);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 10508);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 10509);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5402);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 5403);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 5404);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -icafegrepxe.
pub(crate) fn n14256(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12230);
	s.is_empty()
}

/// Words ending -sutcafegrepxe.
pub(crate) fn n14257(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12231);
	s.is_empty()
}

/// Words ending -rpxe.
pub(crate) fn n2649(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2581);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 5616);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 5617);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5618);
			match s {
				[] => true,
				['b', 'r', 'a', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 10585);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 11628);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11629);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 10586);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['e', 'n', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 11630);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11631);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 10587);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 8614);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -upxe.
pub(crate) fn n2650(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2582);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 5619);
			s.is_empty()
		}
		['g', 'n', 'a', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 8615);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 8616);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 8617);
					s.is_empty()
				}
				['x', '\u{1}', s @ ..] => {
					super::push(out, 8618);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 5620);
			s.is_empty()
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 8619);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 8620);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 5621);
			s.is_empty()
		}
		['t', 'u', 'r', s @ ..] => {
			super::push(out, 5622);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -idupmi.
pub(crate) fn n9764(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8821);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 11667);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 11668);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nupmi.
pub(crate) fn n6354(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5945);
	match s {
		[] => true,
		['c', 't', 'a', 't', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 8822);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 10692);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 10693);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rupmi.
pub(crate) fn n6355(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5946);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 8823);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 8824);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -erfni.
pub(crate) fn n6456(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6040);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 8898);
			s.is_empty()
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 10741);
					match s {
						[] => true,
						['n', 's', s @ ..] => {
							super::push(out, 11679);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 11680);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 10742);
					s.is_empty()
				}
				_ => false,
			}
		}
		['q', s @ ..] => {
			super::push(out, 8899);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -irfni.
pub(crate) fn n6457(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6041);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 8900);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 8901);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -urfni.
pub(crate) fn n6459(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6043);
	s.is_empty()
}

/// Words ending -ufni.
pub(crate) fn n3034(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2941);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 6044);
			s.is_empty()
		}
		['m', 'a', s @ ..] => {
			super::push(out, 6045);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 6046);
			s.is_empty()
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 8902);
					s.is_empty()
				}
				['u', 'r', s @ ..] => {
					super::push(out, 8903);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -eusni.
pub(crate) fn n6570(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6143);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 8972);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 8973);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 8974);
			s.is_empty()
		}
		['t', 'u', s @ ..] => {
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 10777);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 10778);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tusni.
pub(crate) fn n6576(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6149);
	match s {
		[] => true,
		['u', s @ ..] => {
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 10779);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 10780);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -utcnuiretni.
pub(crate) fn n14258(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 12262);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 12263);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -acirtni.
pub(crate) fn n12267(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['n', 's', s @ ..] => {
			super::push(out, 11696);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 11697);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -utirtni.
pub(crate) fn n9975(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 10790);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 10791);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ortni.
pub(crate) fn n6589(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6160);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 8996);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 8997);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 8998);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 10792);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 10793);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 10794);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -utni.
pub(crate) fn n3099(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3005);
	s.is_empty()
}

/// Words ending -ifrep.
pub(crate) fn n7291(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6762);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 9355);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 9356);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 9357);
			match s {
				[] => true,
				['e', 'n', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 11766);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 11767);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 9358);
			match s {
				[] => true,
				['u', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 11768);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 11769);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ofrep.
pub(crate) fn n7293(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6764);
	match s {
		[] => true,
		['t', s @ ..] => {
			super::push(out, 9360);
			match s {
				[] => true,
				['u', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 11770);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 11771);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rfrep.
pub(crate) fn n7294(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6765);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9361);
			match s {
				[] => true,
				['c', 't', 'u', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 11772);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 11773);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 9362);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ufrep.
pub(crate) fn n7295(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6766);
	match s {
		[] => true,
		['n', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 10973);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 10974);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 10975);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', 'u', 'r', s @ ..] => {
			super::push(out, 9363);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -grep.
pub(crate) fn n3705(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3558);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 9364);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 9365);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 6767);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hrep.
pub(crate) fn n3706(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3559);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 6768);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 9366);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 9367);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -irep.
pub(crate) fn n3707(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3560);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 6769);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 9368);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 9369);
					match s {
						[] => true,
						['l', 'o', 's', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 11774);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 11775);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 6770);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 6771);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 6772);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 9370);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 9371);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 6773);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 6774);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 9372);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 9373);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 6775);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 10976);
							match s {
								[] => true,
								['n', 's', s @ ..] => {
									super::push(out, 11776);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 11777);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 10977);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 10978);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 6776);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lrep.
pub(crate) fn n3708(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3561);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6777);
			match s {
				[] => true,
				['b', 'e', 'n', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 10979);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 10980);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 's', 'u', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 10981);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 10982);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 10983);
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 11778);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11779);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 6778);
			match s {
				[] => true,
				['c', 't', 'u', 'r', s @ ..] => {
					super::push(out, 9374);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 9375);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 6779);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 6780);
			match s {
				[] => true,
				['c', 'i', 'd', 'u', 'l', s @ ..] => {
					super::push(out, 9376);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 9377);
					s.is_empty()
				}
				['t', 'u', 'r', s @ ..] => {
					super::push(out, 9378);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mrep.
pub(crate) fn n3709(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3562);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6781);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 9379);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 9380);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 9381);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 12113);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 12114);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 11780);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 10984);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 10985);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 6782);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9382);
					s.is_empty()
				}
				['n', 's', 'u', 'r', s @ ..] => {
					super::push(out, 9383);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 9384);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 11781);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 11782);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 6783);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 6784);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 9385);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 11783);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11784);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 6785);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 9386);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 9387);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 9388);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', 's', s @ ..] => {
									super::push(out, 11785);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 11786);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nrep.
pub(crate) fn n3710(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3563);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 6786);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 9389);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 11787);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11788);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 6787);
			match s {
				[] => true,
				['c', 'i', s @ ..] => {
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 10986);
							match s {
								[] => true,
								['s', 'i', 's', s @ ..] => {
									super::push(out, 11789);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 10987);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 6788);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 9390);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 9391);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 9392);
					s.is_empty()
				}
				['t', 'u', 'r', s @ ..] => {
					super::push(out, 9393);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 6789);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -orep.
pub(crate) fn n3711(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3564);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 6790);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 6791);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 6792);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 6793);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -prep.
pub(crate) fn n3712(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3565);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6794);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 9394);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', 's', s @ ..] => {
									super::push(out, 11790);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 11791);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 9395);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 6795);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 9396);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 10988);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 10989);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 12115);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 12116);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 9397);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 6796);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 6797);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 9398);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 9399);
					match s {
						[] => true,
						['a', 'n', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 11792);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11793);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 11794);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11795);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 6798);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 9400);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 9401);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 9402);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -qrep.
pub(crate) fn n3713(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3566);
	match s {
		[] => true,
		['u', 'i', s @ ..] => {
			match s {
				[] => true,
				['r', 'e', 'n', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 10990);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 10991);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'i', 't', 'u', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 10992);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 10993);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rrep.
pub(crate) fn n3714(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3567);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 6799);
			match s {
				[] => true,
				['c', 't', 'u', 's', s @ ..] => {
					super::push(out, 9403);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 9404);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 6800);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 6801);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 6802);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -srep.
pub(crate) fn n3715(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3568);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6803);
			match s {
				[] => true,
				['l', 'u', 't', 'a', 't', s @ ..] => {
					super::push(out, 9405);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 11796);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11797);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 6804);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 10994);
							s.is_empty()
						}
						['u', 't', 'a', 't', s @ ..] => {
							super::push(out, 10995);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 12117);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 12118);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 6805);
			match s {
				[] => true,
				['v', s @ ..] => {
					super::push(out, 9406);
					match s {
						[] => true,
						['e', 'r', 'a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 11798);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 11799);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 6806);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 6807);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 9407);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 9408);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 11800);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 11801);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 9409);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 6808);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 9410);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 9411);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 10996);
							match s {
								[] => true,
								['a', 'c', s @ ..] => {
									super::push(out, 11802);
									match s {
										[] => true,
										['i', s @ ..] => {
											match s {
												[] => true,
												['o', s @ ..] => {
													super::push(out, 12232);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 12233);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 10997);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 6809);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9412);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 9413);
					match s {
						[] => true,
						['l', 'l', 'a', s @ ..] => {
							match s {
								[] => true,
								['n', 's', s @ ..] => {
									super::push(out, 11803);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 11804);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 9414);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 6810);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9415);
					match s {
						[] => true,
						['d', 'e', 'n', 'd', s @ ..] => {
							super::push(out, 10998);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 11805);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 11806);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 9416);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -trep.
pub(crate) fn n3716(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3569);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 6811);
			match s {
				[] => true,
				['n', 's', s @ ..] => {
					super::push(out, 9417);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 11807);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11808);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 6812);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 9418);
					match s {
						[] => true,
						['e', 's', 'c', 'e', 'n', 'd', s @ ..] => {
							super::push(out, 10999);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 9419);
					match s {
						[] => true,
						['a', 'c', s @ ..] => {
							super::push(out, 11000);
							match s {
								[] => true,
								['i', s @ ..] => {
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 12119);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 12120);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 6813);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 11001);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 11002);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 11003);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 6814);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 9420);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 9421);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 11809);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11810);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -urep.
pub(crate) fn n3717(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3570);
	match s {
		[] => true,
		['n', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 9422);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 9423);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 9424);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 9425);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 6815);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 6816);
			s.is_empty()
		}
		['v', 'i', 'a', 'n', 'u', 's', '\u{1}', s @ ..] => {
			super::push(out, 6817);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vrep.
pub(crate) fn n3718(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3571);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6818);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 9426);
					match s {
						[] => true,
						['e', 'n', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 11811);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11812);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 9427);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 9428);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 11813);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11814);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 6819);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 6820);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 9429);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 11815);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 11816);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -fearp.
pub(crate) fn n7441(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6899);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 9494);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 9495);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 11041);
					match s {
						[] => true,
						['e', 'n', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 12133);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 12134);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 9496);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 11042);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 12240);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 12241);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'u', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 12242);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 12243);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 11043);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 11044);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rusivagearp.
pub(crate) fn n12620(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11045);
	s.is_empty()
}

/// Words ending -iearp.
pub(crate) fn n7443(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6901);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9498);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 9499);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 9500);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 9501);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -learp.
pub(crate) fn n7444(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6902);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 9502);
			match s {
				[] => true,
				['c', 't', 'u', 'r', s @ ..] => {
					super::push(out, 11046);
					s.is_empty()
				}
				['g', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', 'd', s @ ..] => {
									super::push(out, 12135);
									s.is_empty()
								}
								['t', 'u', 'r', s @ ..] => {
									super::push(out, 12136);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 11833);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 9503);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 11047);
					match s {
						[] => true,
						['i', 'n', 'a', 'r', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 11834);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 9504);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 9505);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 11048);
					match s {
						[] => true,
						['e', 'n', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 12137);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 12138);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 11049);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mearp.
pub(crate) fn n7445(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6903);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9506);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 9507);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 9508);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 9509);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 11050);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 11835);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 11836);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 11837);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 11051);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 9510);
			match s {
				[] => true,
				['n', 'i', s @ ..] => {
					match s {
						[] => true,
						['e', 'n', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 12139);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 12140);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 12141);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 12142);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nearp.
pub(crate) fn n7446(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6904);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 9511);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 9512);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 9513);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 11052);
					match s {
						[] => true,
						['c', 'e', 'n', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 12143);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 12144);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 11053);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 9514);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 11054);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 11055);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

