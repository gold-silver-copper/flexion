use flexion::Path;

/// Words ending -igkeit.
pub(crate) fn n22034(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['b', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 41412);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 41413);
							match s {
								[] => true,
								['g', 'r', s @ ..] => {
									super::push(out, 43517);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 37468);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 37469);
					match s {
						[] => true,
						['ä', 'l', 'g', 'r', s @ ..] => {
							super::push(out, 41414);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 30959);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 37470);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 41415);
							match s {
								[] => true,
								['f', 'z', s @ ..] => {
									super::push(out, 43518);
									s.is_empty()
								}
								['w', 'h', 'c', 's', 'e', 'g', s @ ..] => {
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 44465);
											s.is_empty()
										}
										['h', 'c', 's', s @ ..] => {
											super::push(out, 44466);
											s.is_empty()
										}
										['l', 'l', 'a', 'h', 'c', 's', 'r', 'e', 'p', s @ ..] => {
											super::push(out, 44467);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 44468);
											s.is_empty()
										}
										['t', 'h', 'c', 'i', 'l', '-', s @ ..] => {
											super::push(out, 44469);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ä', s @ ..] => {
							super::push(out, 41416);
							match s {
								[] => true,
								['t', 's', s @ ..] => {
									match s {
										[] => true,
										['e', 'b', 's', s @ ..] => {
											super::push(out, 44470);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 44471);
											s.is_empty()
										}
										['n', 'a', 'n', s @ ..] => {
											super::push(out, 44472);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 44473);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ü', s @ ..] => {
							super::push(out, 41417);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'ü', 'w', s @ ..] => {
					match s {
						[] => true,
						['b', 'u', 'a', 'l', 'g', 'n', s @ ..] => {
							super::push(out, 41418);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 41419);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 41420);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 37471);
					match s {
						[] => true,
						['e', 'r', 'f', 's', 'n', s @ ..] => {
							super::push(out, 41421);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ü', s @ ..] => {
					super::push(out, 37472);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 41422);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 37473);
					match s {
						[] => true,
						['i', 'r', s @ ..] => {
							super::push(out, 41423);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'ö', s @ ..] => {
					super::push(out, 37474);
					match s {
						[] => true,
						['k', s @ ..] => {
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 43519);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 43520);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 37475);
					match s {
						[] => true,
						['ä', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 43521);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 43522);
									match s {
										[] => true,
										['t', s @ ..] => {
											super::push(out, 44474);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', 'r', s @ ..] => {
							super::push(out, 41424);
							s.is_empty()
						}
						['ä', 'h', 'b', 'a', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 43523);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 43524);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ü', s @ ..] => {
					super::push(out, 37476);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 37477);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 41425);
							s.is_empty()
						}
						['s', 't', s @ ..] => {
							super::push(out, 41426);
							s.is_empty()
						}
						['ä', 'l', s @ ..] => {
							super::push(out, 41427);
							s.is_empty()
						}
						['ü', s @ ..] => {
							super::push(out, 41428);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', 'f', s @ ..] => {
					match s {
						[] => true,
						['e', 's', s @ ..] => {
							super::push(out, 41429);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 41430);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 41431);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 43525);
									s.is_empty()
								}
								['g', 'n', 'u', 's', s @ ..] => {
									super::push(out, 43526);
									s.is_empty()
								}
								['n', 'o', 'i', 't', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 44475);
											s.is_empty()
										}
										['k', 'n', s @ ..] => {
											super::push(out, 44476);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 30960);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['d', 'd', s @ ..] => {
							super::push(out, 41432);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['d', 'e', s @ ..] => {
									super::push(out, 43527);
									s.is_empty()
								}
								['m', 'r', s @ ..] => {
									super::push(out, 43528);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 43529);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => super::t16::n39653(s, out),
				['i', s @ ..] => super::t16::n39654(s, out),
				['l', s @ ..] => super::t16::n39655(s, out),
				_ => false,
			}
		}
		['m', 'm', s @ ..] => super::t16::n32403(s, out),
		['n', s @ ..] => super::t16::n32404(s, out),
		['r', s @ ..] => super::t16::n32405(s, out),
		['s', s @ ..] => super::t16::n32406(s, out),
		['t', s @ ..] => super::t16::n32407(s, out),
		['u', s @ ..] => super::t16::n32408(s, out),
		['z', s @ ..] => super::t16::n32409(s, out),
		['ß', s @ ..] => super::t16::n32410(s, out),
		_ => false,
	}
}

/// Words ending -chkeit.
pub(crate) fn n32411(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['i', 'l', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 43557);
							s.is_empty()
						}
						['i', 'e', 'l', s @ ..] => {
							super::push(out, 43558);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 43559);
							match s {
								[] => true,
								['e', 't', 's', 'r', 'e', 'b', s @ ..] => {
									super::push(out, 44520);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 41467);
					match s {
						[] => true,
						['e', 'i', s @ ..] => {
							super::push(out, 43560);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 44860);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 44861);
											match s {
												[] => true,
												['f', s @ ..] => {
													match s {
														[] => true,
														['n', 'e', s @ ..] => super::t16::n48075(s, out),
														['o', s @ ..] => super::t16::n48076(s, out),
														['r', 'e', 'd', 'n', 'ä', s @ ..] => super::t16::n48077(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['f', s @ ..] => {
											super::push(out, 44862);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', 'e', 'r', 'f', 't', s @ ..] => {
									super::push(out, 44521);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 41468);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 43561);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 41469);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 43562);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 44522);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 43563);
							match s {
								[] => true,
								['i', 'r', 'd', 'u', s @ ..] => {
									super::push(out, 44523);
									s.is_empty()
								}
								['ä', s @ ..] => {
									match s {
										[] => true,
										['g', s @ ..] => {
											super::push(out, 44863);
											s.is_empty()
										}
										['h', s @ ..] => {
											super::push(out, 44864);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 44865);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 43564);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 43565);
							match s {
								[] => true,
								['a', 't', 'h', s @ ..] => {
									super::push(out, 44524);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ä', 'r', 't', 'r', 'e', 'v', 'e', 'i', s @ ..] => {
							super::push(out, 43566);
							s.is_empty()
						}
						['ö', s @ ..] => {
							super::push(out, 43567);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', 'r', s @ ..] => {
									super::push(out, 44525);
									s.is_empty()
								}
								['e', s @ ..] => super::t16::n47534(s, out),
								['i', s @ ..] => {
									super::push(out, 44526);
									s.is_empty()
								}
								['s', 'n', 'e', 'm', 't', s @ ..] => {
									super::push(out, 44527);
									s.is_empty()
								}
								['ä', s @ ..] => super::t16::n47537(s, out),
								_ => false,
							}
						}
						['ö', s @ ..] => super::t16::n46473(s, out),
						_ => false,
					}
				}
				['i', 'e', 't', 'r', 'a', 'p', 'r', s @ ..] => super::t16::n44133(s, out),
				['k', s @ ..] => super::t16::n44134(s, out),
				['m', s @ ..] => super::t16::n44135(s, out),
				['n', s @ ..] => super::t16::n44136(s, out),
				['r', s @ ..] => super::t16::n44137(s, out),
				['s', s @ ..] => super::t16::n44138(s, out),
				['t', s @ ..] => super::t16::n44139(s, out),
				['u', s @ ..] => super::t16::n44140(s, out),
				['z', 't', s @ ..] => super::t16::n44141(s, out),
				['ß', 'e', 'i', 'l', 'h', 'c', 's', 's', 'u', 'a', 'n', s @ ..] => super::t16::n44142(s, out),
				_ => false,
			}
		}
		['s', 'i', s @ ..] => super::t16::n39691(s, out),
		_ => false,
	}
}

/// Words ending -barkeit.
pub(crate) fn n32422(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['b', 'r', 'e', s @ ..] => {
			super::push(out, 37518);
			s.is_empty()
		}
		['d', s @ ..] => {
			match s {
				[] => true,
				['i', 'e', 'h', s @ ..] => {
					super::push(out, 41479);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 41480);
					match s {
						[] => true,
						['e', 'w', 'r', 'e', 'v', 'r', s @ ..] => {
							super::push(out, 43586);
							s.is_empty()
						}
						['u', 'w', 'r', 'e', 'v', 'n', s @ ..] => {
							super::push(out, 43587);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 41481);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 41482);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 43588);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 43589);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ü', s @ ..] => {
					super::push(out, 41483);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 37519);
			match s {
				[] => true,
				['c', 's', 'r', 'r', s @ ..] => {
					super::push(out, 41484);
					s.is_empty()
				}
				['e', 'g', s @ ..] => {
					super::push(out, 41485);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 41486);
					s.is_empty()
				}
				['h', 'e', 'f', 'n', s @ ..] => {
					super::push(out, 41487);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 41488);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 41489);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 41490);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 41491);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 41492);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 41493);
					match s {
						[] => true,
						['e', 'r', s @ ..] => {
							super::push(out, 43590);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 37520);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['d', 'n', s @ ..] => {
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 44558);
									s.is_empty()
								}
								['ä', s @ ..] => {
									super::push(out, 44559);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 43591);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 44560);
									s.is_empty()
								}
								['l', 'a', s @ ..] => {
									super::push(out, 44561);
									s.is_empty()
								}
								['z', s @ ..] => {
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 44874);
											s.is_empty()
										}
										['u', 'd', 'n', s @ ..] => {
											super::push(out, 44875);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', 'e', s @ ..] => {
					super::push(out, 41494);
					s.is_empty()
				}
				['ä', s @ ..] => {
					super::push(out, 41495);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 41496);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 41497);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 41498);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 41499);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 41500);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 41501);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 41502);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 41503);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 41504);
					match s {
						[] => true,
						['a', 't', s @ ..] => {
							super::push(out, 43592);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 37521);
			s.is_empty()
		}
		['z', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 41505);
					s.is_empty()
				}
				['t', 'u', 'n', 'e', s @ ..] => {
					super::push(out, 41506);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -erkeit.
pub(crate) fn n32423(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['b', 'u', 'a', 's', 'n', s @ ..] => {
			super::push(out, 37522);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 37523);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 37524);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 41507);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -management.
pub(crate) fn n46564(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 44569);
			match s {
				[] => true,
				['t', 'e', s @ ..] => {
					super::push(out, 44876);
					match s {
						[] => true,
						['s', 's', 'a', '\u{1}', s @ ..] => {
							super::push(out, 44982);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 44877);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 44570);
			s.is_empty()
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 44878);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 44879);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'e', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 44880);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 44881);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 44882);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 44571);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 44883);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 44884);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 44572);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 44573);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 44574);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 44885);
					s.is_empty()
				}
				['t', 'i', 'e', 'k', s @ ..] => {
					super::push(out, 44886);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 44887);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 44888);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 44575);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lement.
pub(crate) fn n32733(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31248);
	match s {
		[] => true,
		['b', 'u', 'e', 'm', '\u{1}', s @ ..] => {
			super::push(out, 37713);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 37714);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 41636);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 41637);
					s.is_empty()
				}
				['d', 'n', 'u', 'r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 41638);
					s.is_empty()
				}
				['h', 'c', s @ ..] => {
					match s {
						[] => true,
						['a', 'r', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 43645);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 43646);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 41639);
					s.is_empty()
				}
				['k', 'c', 'u', 'm', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 41640);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 43647);
							s.is_empty()
						}
						['e', 'i', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 43648);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'i', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 41641);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 44889);
											s.is_empty()
										}
										['u', 'f', '\u{1}', s @ ..] => {
											super::push(out, 44890);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', 'd', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 44576);
									s.is_empty()
								}
								['n', 'i', 'h', 'c', 's', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 44577);
									s.is_empty()
								}
								['p', 'p', 'u', 'r', 'g', 't', 'p', 'u', 'a', 'h', '\u{1}', s @ ..] => {
									super::push(out, 44578);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 43649);
							s.is_empty()
						}
						['o', 't', 's', 'e', 'w', '\u{1}', s @ ..] => {
							super::push(out, 43650);
							s.is_empty()
						}
						['r', 'e', 'k', '\u{1}', s @ ..] => {
							super::push(out, 43651);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['k', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 43652);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 43653);
							s.is_empty()
						}
						['r', 'e', 't', 'e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 43654);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 43655);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 41642);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 43656);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 43657);
							s.is_empty()
						}
						['u', 't', 'k', 'u', 'r', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 43658);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 43659);
							s.is_empty()
						}
						['g', 'n', 'u', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									match s {
										[] => true,
										['l', 'i', 'b', '\u{1}', s @ ..] => {
											super::push(out, 44891);
											s.is_empty()
										}
										['n', 'i', 'b', 'r', 'e', 'v', '\u{1}', s @ ..] => {
											super::push(out, 44892);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 44579);
									s.is_empty()
								}
								['t', 'f', 'ü', 'l', '\u{1}', s @ ..] => {
									super::push(out, 44580);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'l', 'a', 'f', 'u', 'z', '\u{1}', s @ ..] => {
							super::push(out, 43660);
							s.is_empty()
						}
						['n', 'o', 'i', s @ ..] => {
							match s {
								[] => true,
								['t', 'k', 'n', 'u', 'f', '\u{1}', s @ ..] => {
									super::push(out, 44581);
									s.is_empty()
								}
								['x', 'e', 'l', 'f', '\u{1}', s @ ..] => {
									super::push(out, 44582);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'i', 'e', 'h', 'n', 'i', 'e', '\u{1}', s @ ..] => {
							super::push(out, 43661);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['o', 'v', 'i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 43662);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 43663);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'a', 'b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 43664);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 43665);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', 'i', 'e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 41643);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 37715);
			match s {
				[] => true,
				['e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 41644);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 37716);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 41645);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 'm', s @ ..] => {
			super::push(out, 37717);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sement.
pub(crate) fn n32736(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31251);
	match s {
		[] => true,
		['s', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 41647);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 41648);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zement.
pub(crate) fn n32738(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31253);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 37721);
			match s {
				[] => true,
				['n', 'a', 'l', 't', 'r', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 41649);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 37722);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 41650);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 37723);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gment.
pub(crate) fn n22419(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21772);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 31254);
			match s {
				[] => true,
				['r', 'f', '\u{1}', s @ ..] => {
					super::push(out, 37724);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 31255);
			s.is_empty()
		}
		['i', 'p', s @ ..] => {
			match s {
				[] => true,
				['b', 'r', 'a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 37725);
					s.is_empty()
				}
				['ß', s @ ..] => {
					super::push(out, 37726);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hment.
pub(crate) fn n22420(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21773);
	s.is_empty()
}

/// Words ending -iment.
pub(crate) fn n22421(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21774);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 31256);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 37727);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 37728);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 31257);
			match s {
				[] => true,
				['e', 'r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 41651);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 41652);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 41653);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 41654);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 41655);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 31258);
			match s {
				[] => true,
				['p', 'm', 'o', 'k', 'n', 'e', 's', s @ ..] => {
					super::push(out, 37729);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 31259);
			match s {
				[] => true,
				['i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 37730);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 31260);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 37731);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 31261);
			match s {
				[] => true,
				['e', 'p', 'x', 'e', 's', 't', s @ ..] => {
					super::push(out, 37732);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 31262);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 37733);
					match s {
						[] => true,
						['e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 41656);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 37734);
					match s {
						[] => true,
						['a', 'p', 'm', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 41657);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mment.
pub(crate) fn n22422(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21775);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 31263);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nment.
pub(crate) fn n22423(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21776);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 31264);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oment.
pub(crate) fn n22424(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21777);
	match s {
		[] => true,
		['m', s @ ..] => {
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 37735);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 41658);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 41659);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 43666);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pment.
pub(crate) fn n22425(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21778);
	s.is_empty()
}

/// Words ending -rment.
pub(crate) fn n22426(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21779);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 37736);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 37737);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 31265);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tment.
pub(crate) fn n22427(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21780);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 31266);
			match s {
				[] => true,
				['a', 'n', 'e', 'e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 37738);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 31267);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ument.
pub(crate) fn n22428(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21781);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 31268);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 37739);
					s.is_empty()
				}
				['r', 'a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 41660);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 41661);
							s.is_empty()
						}
						['f', 'u', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 41662);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 41663);
							s.is_empty()
						}
						['l', 'e', 'i', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 41664);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['e', 'g', 'e', 'g', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 44583);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 44584);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', 'e', 'h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 43667);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 41665);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['f', 'u', 'a', 'k', 'r', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 43668);
									s.is_empty()
								}
								['g', 'n', 'i', 'l', 'b', 'e', 'i', 'l', '\u{1}', s @ ..] => {
									super::push(out, 43669);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'p', 'u', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 41666);
							s.is_empty()
						}
						['z', 'n', 'e', 'u', 'q', 'e', 's', 'n', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 41667);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 31269);
			match s {
				[] => true,
				['o', 'd', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 41668);
							s.is_empty()
						}
						['e', 's', 'i', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 41669);
							s.is_empty()
						}
						['h', 'c', 'a', 'r', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 41670);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 41671);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['a', 'n', 'i', 'g', 'i', 'r', 'o', '\u{1}', s @ ..] => {
									super::push(out, 43670);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 43671);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'i', 'e', 'h', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 41672);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 41673);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['g', 'n', 'u', 'r', 'e', s @ ..] => {
									match s {
										[] => true,
										['d', 'r', 'o', 'f', 'n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 44585);
											s.is_empty()
										}
										['i', 'g', 'e', 'r', '\u{1}', s @ ..] => {
											super::push(out, 44586);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 'u', 'l', 'h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 43672);
									s.is_empty()
								}
								['t', 'h', 'c', 'i', 'r', 'e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 43673);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'f', 'i', 'r', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 41674);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 41675);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 31270);
			match s {
				[] => true,
				['t', 's', 'n', 'i', s @ ..] => {
					match s {
						[] => true,
						['n', 'e', 't', 's', 'a', 't', '\u{1}', s @ ..] => {
							super::push(out, 41676);
							s.is_empty()
						}
						['r', 'e', 't', 'l', s @ ..] => {
							super::push(out, 41677);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 43674);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 44587);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 43675);
									s.is_empty()
								}
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
			super::push(out, 31271);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 37740);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word schlusswort.
pub(crate) fn n44442(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41746);
	s.is_empty()
}

/// Words ending -tswort.
pub(crate) fn n32974(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31467);
	s.is_empty()
}

/// Words ending -twort.
pub(crate) fn n22676(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22020);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 31468);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 31469);
			match s {
				[] => true,
				['c', 'a', s @ ..] => {
					super::push(out, 37882);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 31470);
			match s {
				[] => true,
				['e', 'l', '\u{1}', s @ ..] => {
					super::push(out, 37883);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 31471);
			match s {
				[] => true,
				['e', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 37884);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 31472);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 37885);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 37886);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 37887);
					s.is_empty()
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 31473);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uwort.
pub(crate) fn n22677(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22021);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 31474);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zwort.
pub(crate) fn n22678(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22022);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 31475);
			match s {
				[] => true,
				['e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 37888);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['e', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 37889);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 37890);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'e', 'h', '\u{1}', s @ ..] => {
			super::push(out, 31476);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ßwort.
pub(crate) fn n22679(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 31477);
			match s {
				[] => true,
				['e', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 37891);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 31478);
			match s {
				[] => true,
				['l', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 37892);
					s.is_empty()
				}
				['r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 37893);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -stest.
pub(crate) fn n22785(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22124);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 31555);
			match s {
				[] => true,
				['g', 'b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 37954);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 31556);
			match s {
				[] => true,
				['i', 'a', '\u{1}', s @ ..] => {
					super::push(out, 37955);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 31557);
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 41770);
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 43694);
									match s {
										[] => true,
										['g', 'i', 'e', '\u{1}', s @ ..] => {
											super::push(out, 44594);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 43695);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 43696);
									match s {
										[] => true,
										['s', 'a', 'l', 'u', 'z', 'l', 'u', 'h', 'c', 's', 'h', 'c', 'o', 'h', '\u{1}', s @ ..] => {
											super::push(out, 44595);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 43697);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', 'c', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 37956);
					match s {
						[] => true,
						['e', 'l', 'g', 'r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 41771);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 37957);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'x', 'a', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 31558);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 31559);
			match s {
				[] => true,
				['o', 'i', s @ ..] => {
					match s {
						[] => true,
						['s', 'o', 'r', 'r', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 41772);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['k', s @ ..] => {
											super::push(out, 44596);
											s.is_empty()
										}
										['l', 'e', 'r', 'r', 'o', 'k', '\u{1}', s @ ..] => {
											super::push(out, 44597);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 44598);
											s.is_empty()
										}
										_ => false,
									}
								}
								['k', s @ ..] => {
									super::push(out, 43698);
									s.is_empty()
								}
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
			super::push(out, 31560);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 37958);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 31561);
			match s {
				[] => true,
				['f', 'a', 'h', 'c', 's', 'r', 'e', s @ ..] => {
					match s {
						[] => true,
						['g', 'n', 'a', 'w', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 41773);
							s.is_empty()
						}
						['t', 'a', 'v', '\u{1}', s @ ..] => {
							super::push(out, 41774);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 31562);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 37959);
					s.is_empty()
				}
				['r', 'i', 'v', '\u{1}', s @ ..] => {
					super::push(out, 37960);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sttest.
pub(crate) fn n33090(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31567);
	s.is_empty()
}

/// Words ending -uttest.
pub(crate) fn n33091(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31568);
	s.is_empty()
}

/// Words ending -xttest.
pub(crate) fn n33092(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31569);
	s.is_empty()
}

/// Words ending -ztest.
pub(crate) fn n22787(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22126);
	s.is_empty()
}

/// Words ending -ßtest.
pub(crate) fn n22788(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22127);
	s.is_empty()
}

/// Words ending -ätest.
pub(crate) fn n22789(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22128);
	s.is_empty()
}

/// Words ending -nalität.
pub(crate) fn n40430(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 41904);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 41905);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 43731);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 43732);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 43733);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 43734);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 44897);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 44898);
									s.is_empty()
								}
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
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['s', 'n', 'e', 'm', 'i', 'd', 'r', s @ ..] => {
							super::push(out, 44605);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['k', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 44985);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 44986);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 44899);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 43735);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 43736);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 44606);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ralität.
pub(crate) fn n40431(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38156);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 43737);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 43738);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'm', s @ ..] => {
			super::push(out, 41906);
			s.is_empty()
		}
		['t', 'n', s @ ..] => {
			super::push(out, 41907);
			s.is_empty()
		}
		['u', 'l', s @ ..] => {
			super::push(out, 41908);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -talität.
pub(crate) fn n40433(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'f', s @ ..] => {
			super::push(out, 41909);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 41910);
			s.is_empty()
		}
		['n', 'e', 'm', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 43739);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 43740);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 41911);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 41912);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ualität.
pub(crate) fn n40434(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38158);
	match s {
		[] => true,
		['d', 'i', 'v', 'i', 'd', 'n', s @ ..] => {
			super::push(out, 41913);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 41914);
			s.is_empty()
		}
		['q', s @ ..] => {
			super::push(out, 41915);
			match s {
				[] => true,
				['l', 'e', 't', s @ ..] => {
					super::push(out, 43741);
					s.is_empty()
				}
				['r', 'e', 's', 's', 'a', 'w', 'k', s @ ..] => {
					super::push(out, 43742);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 43743);
					match s {
						[] => true,
						['g', 'n', 'u', 't', s @ ..] => {
							super::push(out, 44607);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 44608);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 44609);
							match s {
								[] => true,
								['e', 'z', s @ ..] => {
									super::push(out, 44900);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 'f', 'u', 'l', 'm', 'u', 'a', 'r', 'n', s @ ..] => {
					super::push(out, 43744);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 41916);
			s.is_empty()
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 43745);
					s.is_empty()
				}
				['k', 'a', s @ ..] => {
					super::push(out, 43746);
					s.is_empty()
				}
				['n', 'e', 'v', s @ ..] => {
					super::push(out, 43747);
					s.is_empty()
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 41917);
			match s {
				[] => true,
				['e', 's', s @ ..] => {
					match s {
						[] => true,
						['o', 'm', s @ ..] => {
							super::push(out, 44610);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 44611);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ilität.
pub(crate) fn n33404(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31841);
	match s {
		[] => true,
		['b', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['i', 'r', s @ ..] => {
							super::push(out, 43748);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 44612);
									s.is_empty()
								}
								['s', 's', s @ ..] => {
									super::push(out, 44613);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['g', 'n', s @ ..] => {
							super::push(out, 43749);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['n', 'e', 's', 'n', s @ ..] => {
									super::push(out, 44614);
									s.is_empty()
								}
								['s', 'e', s @ ..] => {
									super::push(out, 44615);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 44616);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 44617);
									match s {
										[] => true,
										['p', 'm', 'o', 'k', 'r', s @ ..] => {
											super::push(out, 44901);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									super::push(out, 44618);
									s.is_empty()
								}
								_ => false,
							}
						}
						['x', 'e', 'l', 'f', s @ ..] => {
							super::push(out, 43750);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 38161);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 38162);
			s.is_empty()
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['a', 'l', s @ ..] => {
					super::push(out, 41919);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 41920);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 41921);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 38163);
			s.is_empty()
		}
		['z', 'a', 'f', s @ ..] => {
			super::push(out, 38164);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -olität.
pub(crate) fn n33405(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31842);
	s.is_empty()
}

/// Words ending -mität.
pub(crate) fn n23148(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22470);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 31843);
			s.is_empty()
		}
		['i', 't', 'i', s @ ..] => {
			super::push(out, 31844);
			s.is_empty()
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 38165);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 43751);
									s.is_empty()
								}
								['n', 'o', 'k', s @ ..] => {
									match s {
										[] => true,
										['s', s @ ..] => {
											super::push(out, 44619);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 44620);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 41922);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', 'n', 'o', 'n', 'a', s @ ..] => {
			super::push(out, 31845);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nität.
pub(crate) fn n23149(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 31846);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 38166);
					s.is_empty()
				}
				['m', 'u', 'h', 'n', s @ ..] => {
					super::push(out, 38167);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 38168);
					s.is_empty()
				}
				['t', 'l', s @ ..] => {
					super::push(out, 38169);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 31847);
			match s {
				[] => true,
				['g', 'o', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 41923);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 43752);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 41924);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 41925);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 41926);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 31848);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 38170);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 38171);
					s.is_empty()
				}
				['l', 'u', s @ ..] => {
					super::push(out, 38172);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 31849);
			s.is_empty()
		}
		['r', 'e', 't', 'a', 'r', s @ ..] => {
			super::push(out, 31850);
			s.is_empty()
		}
		['u', 'm', 'm', 'i', 'o', s @ ..] => {
			super::push(out, 31851);
			s.is_empty()
		}
		['ä', 'r', 'e', 'z', s @ ..] => {
			super::push(out, 31852);
			s.is_empty()
		}
		['ö', s @ ..] => {
			super::push(out, 31853);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rität.
pub(crate) fn n23150(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22471);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 38173);
					s.is_empty()
				}
				['i', 'l', s @ ..] => {
					super::push(out, 38174);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 38175);
					match s {
						[] => true,
						['o', 'p', s @ ..] => {
							super::push(out, 41927);
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 43753);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 43754);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 43755);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', 'i', s @ ..] => {
					match s {
						[] => true,
						['b', 't', s @ ..] => {
							super::push(out, 41928);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 41929);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 38176);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 38177);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', 'u', s @ ..] => {
			super::push(out, 31854);
			s.is_empty()
		}
		['e', 't', 's', s @ ..] => {
			super::push(out, 31855);
			s.is_empty()
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 41930);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['f', s @ ..] => {
											super::push(out, 44621);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 44622);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									super::push(out, 43756);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 41931);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 41932);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', 't', s @ ..] => {
			super::push(out, 31856);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sität.
pub(crate) fn n23151(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22472);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 31857);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 41933);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 41934);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 31858);
			match s {
				[] => true,
				['b', 'r', 'e', s @ ..] => {
					super::push(out, 38178);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 38179);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 41935);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 41936);
							s.is_empty()
						}
						['r', 'u', s @ ..] => {
							super::push(out, 41937);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 38180);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 38181);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 41938);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 41939);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 38182);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 31859);
			match s {
				[] => true,
				['e', 'v', 'i', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 41940);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 41941);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tität.
pub(crate) fn n23152(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22473);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 31860);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uität.
pub(crate) fn n23153(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['g', 'i', s @ ..] => {
			super::push(out, 31861);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 31862);
			s.is_empty()
		}
		['n', 'i', 't', 'n', 'o', 'k', 's', s @ ..] => {
			super::push(out, 31863);
			s.is_empty()
		}
		['q', s @ ..] => {
			super::push(out, 31864);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vität.
pub(crate) fn n23154(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'r', s @ ..] => {
			super::push(out, 31865);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 38183);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 38184);
					match s {
						[] => true,
						['s', 'e', 'r', 'g', 'g', s @ ..] => {
							super::push(out, 41942);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 43757);
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
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 43758);
									s.is_empty()
								}
								['g', s @ ..] => {
									match s {
										[] => true,
										['e', 'n', 'o', s @ ..] => {
											super::push(out, 44623);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 44624);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 43759);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 43760);
									s.is_empty()
								}
								['t', 'u', s @ ..] => {
									super::push(out, 43761);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 's', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 44625);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 44626);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 43762);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 43763);
									match s {
										[] => true,
										['n', 'i', s @ ..] => {
											super::push(out, 44627);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 44628);
											match s {
												[] => true,
												['i', s @ ..] => {
													super::push(out, 44902);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 44629);
											match s {
												[] => true,
												['e', s @ ..] => {
													match s {
														[] => true,
														['p', s @ ..] => super::t16::n48048(s, out),
														['t', s @ ..] => super::t16::n48049(s, out),
														_ => false,
													}
												}
												['o', s @ ..] => {
													super::push(out, 44903);
													s.is_empty()
												}
												['t', s @ ..] => {
													super::push(out, 44904);
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', s @ ..] => {
											super::push(out, 44630);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 44631);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									match s {
										[] => true,
										['j', s @ ..] => {
											super::push(out, 44632);
											match s {
												[] => true,
												['b', s @ ..] => {
													match s {
														[] => true,
														['o', s @ ..] => super::t16::n48050(s, out),
														['u', s @ ..] => super::t16::n48051(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['n', s @ ..] => {
											super::push(out, 44633);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 43764);
									match s {
										[] => true,
										['d', s @ ..] => {
											match s {
												[] => true,
												['n', s @ ..] => {
													super::push(out, 44905);
													s.is_empty()
												}
												['o', s @ ..] => {
													super::push(out, 44906);
													s.is_empty()
												}
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
							super::push(out, 41943);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zität.
pub(crate) fn n23155(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 31866);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 38185);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 38186);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 38187);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 41944);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 41945);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 43765);
									s.is_empty()
								}
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
			super::push(out, 31867);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 38188);
					s.is_empty()
				}
				['n', 'o', 'r', 'h', 'c', 'n', s @ ..] => {
					super::push(out, 38189);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 38190);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 41946);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 43766);
									s.is_empty()
								}
								['n', 'e', 'z', 'x', s @ ..] => {
									super::push(out, 43767);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', 'a', s @ ..] => {
					super::push(out, 38191);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 41947);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 41948);
							s.is_empty()
						}
						['s', 'a', 'l', 'e', 's', s @ ..] => {
							super::push(out, 41949);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', 'v', s @ ..] => {
			super::push(out, 31868);
			s.is_empty()
		}
		['ä', s @ ..] => {
			super::push(out, 31869);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word belagerungszustand.
pub(crate) fn n48052(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44991);
	s.is_empty()
}

/// Words ending -hrungszustand.
pub(crate) fn n48053(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44992);
	s.is_empty()
}

/// Words ending -elenwasserstoff.
pub(crate) fn n48054(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44993);
	s.is_empty()
}

/// Words ending -hlenwasserstoff.
pub(crate) fn n48055(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44994);
	match s {
		[] => true,
		['o', 'k', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 45020);
					s.is_empty()
				}
				['n', 'e', 'g', 'o', 'l', 'a', 'h', '\u{1}', s @ ..] => {
					super::push(out, 45021);
					s.is_empty()
				}
				['r', 'o', s @ ..] => {
					match s {
						[] => true,
						['l', 'h', 'c', 'r', 'o', 'u', 'l', 'f', '\u{1}', s @ ..] => {
							super::push(out, 45022);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 45023);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -l-organisation.
pub(crate) fn n48065(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 45003);
	s.is_empty()
}

/// Words ending -heitsorganisation.
pub(crate) fn n48066(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 45004);
	s.is_empty()
}

/// Words ending -nalismus.
pub(crate) fn n43622(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['o', 'i', s @ ..] => {
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 44321);
					s.is_empty()
				}
				['s', 'a', s @ ..] => {
					super::push(out, 44322);
					s.is_empty()
				}
				['t', 'a', s @ ..] => {
					super::push(out, 44323);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 44958);
									s.is_empty()
								}
								['r', 'e', s @ ..] => {
									super::push(out, 44959);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 44768);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 43290);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ralismus.
pub(crate) fn n43623(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 43291);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 43292);
			s.is_empty()
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['l', 'p', 'o', s @ ..] => {
					super::push(out, 44324);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 44769);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 44770);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -talismus.
pub(crate) fn n43624(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['i', 'n', s @ ..] => {
			super::push(out, 43293);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 43294);
			match s {
				[] => true,
				['e', 'm', 'a', s @ ..] => {
					super::push(out, 44325);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ualismus.
pub(crate) fn n43625(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['d', s @ ..] => {
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 44326);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 44327);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 43295);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 43296);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -valismus.
pub(crate) fn n43626(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41031);
	s.is_empty()
}

/// Words ending -lelismus.
pub(crate) fn n39086(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37001);
	s.is_empty()
}

/// Words ending -ilismus.
pub(crate) fn n39087(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 41032);
			s.is_empty()
		}
		['t', 'n', 'a', 'f', s @ ..] => {
			super::push(out, 41033);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -olismus.
pub(crate) fn n39088(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37002);
	match s {
		[] => true,
		['b', 'a', s @ ..] => {
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 43297);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 44328);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 44329);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', 'o', s @ ..] => {
			super::push(out, 41034);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ulismus.
pub(crate) fn n39089(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37003);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 41035);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mismus.
pub(crate) fn n31807(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 41036);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 41037);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 41038);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 43298);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 't', 'x', 'e', 's', s @ ..] => {
					super::push(out, 41039);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 41040);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 41041);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 41042);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 37004);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nismus.
pub(crate) fn n31808(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30440);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 37005);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 41043);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 43299);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 43300);
							s.is_empty()
						}
						['i', 'z', s @ ..] => {
							super::push(out, 43301);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 43302);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 41044);
					match s {
						[] => true,
						['c', 'e', 'm', 's', 'b', s @ ..] => {
							super::push(out, 43303);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 41045);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 43304);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 44330);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 43305);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 44331);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 43306);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', 'x', s @ ..] => {
			super::push(out, 37006);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 41046);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['o', 'r', s @ ..] => {
							super::push(out, 43307);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 43308);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 43309);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 43310);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 41047);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 41048);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 43311);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 43312);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 41049);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['d', 'e', s @ ..] => {
					super::push(out, 41050);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['i', 'v', 'e', 'r', 's', s @ ..] => {
									super::push(out, 44332);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 44333);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['a', 'r', s @ ..] => {
									super::push(out, 44334);
									s.is_empty()
								}
								['k', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 44771);
											s.is_empty()
										}
										['e', 'n', s @ ..] => {
											super::push(out, 44772);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['z', 'i', s @ ..] => {
							super::push(out, 43313);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', 'i', s @ ..] => {
					super::push(out, 41051);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 41052);
					s.is_empty()
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 43314);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 43315);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'h', s @ ..] => {
					super::push(out, 41053);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 43316);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 43317);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 43318);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 41054);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 37007);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 37008);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oismus.
pub(crate) fn n31809(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30441);
	match s {
		[] => true,
		['g', 'e', s @ ..] => {
			super::push(out, 37009);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pismus.
pub(crate) fn n31810(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30442);
	match s {
		[] => true,
		['a', 'i', s @ ..] => {
			super::push(out, 37010);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 37011);
			match s {
				[] => true,
				['r', 't', 'k', s @ ..] => {
					super::push(out, 41055);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rismus.
pub(crate) fn n31811(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30443);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 41056);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 41057);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 41058);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 41059);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 41060);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 43319);
							s.is_empty()
						}
						['i', 'l', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 44335);
									match s {
										[] => true,
										['t', s @ ..] => {
											super::push(out, 44773);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', 'm', s @ ..] => {
									super::push(out, 44336);
									s.is_empty()
								}
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
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 41061);
					s.is_empty()
				}
				['l', 'l', s @ ..] => {
					super::push(out, 41062);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 41063);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 41064);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 41065);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 41066);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 41067);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 41068);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 41069);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 41070);
					match s {
						[] => true,
						['e', 'z', 'o', 'r', s @ ..] => {
							super::push(out, 43320);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 41071);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 37012);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 41072);
					match s {
						[] => true,
						['t', s @ ..] => {
							match s {
								[] => true,
								['n', 'e', s @ ..] => {
									super::push(out, 44337);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 44338);
									match s {
										[] => true,
										['t', 'r', s @ ..] => {
											super::push(out, 44774);
											s.is_empty()
										}
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
					super::push(out, 41073);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 41074);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 43321);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sismus.
pub(crate) fn n31812(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['s', 'a', 'r', 'i', s @ ..] => {
			super::push(out, 37013);
			s.is_empty()
		}
		['ö', s @ ..] => {
			super::push(out, 37014);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tismus.
pub(crate) fn n31813(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30444);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 41075);
					match s {
						[] => true,
						['e', 'r', s @ ..] => {
							super::push(out, 43322);
							s.is_empty()
						}
						['g', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 44339);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 44340);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 44341);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 43323);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 43324);
							s.is_empty()
						}
						['u', 'e', 'h', 'r', 'l', s @ ..] => {
							super::push(out, 43325);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 41076);
					s.is_empty()
				}
				['r', 'o', s @ ..] => {
					super::push(out, 41077);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 41078);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 43326);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 41079);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 41080);
					s.is_empty()
				}
				['k', 'y', s @ ..] => {
					super::push(out, 41081);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 41082);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 41083);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 41084);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 41085);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 41086);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 41087);
					match s {
						[] => true,
						['e', 's', 'i', s @ ..] => {
							super::push(out, 43327);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 41088);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 41089);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 37015);
			s.is_empty()
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['a', 't', 't', s @ ..] => {
					super::push(out, 41090);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 43328);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 43329);
							s.is_empty()
						}
						['s', 'b', s @ ..] => {
							super::push(out, 43330);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 41091);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 41092);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 41093);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 37016);
			s.is_empty()
		}
		['u', 's', s @ ..] => {
			super::push(out, 37017);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -huismus.
pub(crate) fn n31814(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30445);
	s.is_empty()
}

/// Words ending -vismus.
pub(crate) fn n31815(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 37018);
			s.is_empty()
		}
		['i', 't', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 41094);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 41095);
					s.is_empty()
				}
				['k', 'u', 'r', 't', 's', 'n', 'o', 'k', 'e', s @ ..] => {
					super::push(out, 41096);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -exismus.
pub(crate) fn n31816(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30446);
	s.is_empty()
}

/// Words ending -yismus.
pub(crate) fn n31817(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30447);
	s.is_empty()
}

/// Words ending -zismus.
pub(crate) fn n31818(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30448);
	match s {
		[] => true,
		['a', 'n', s @ ..] => {
			super::push(out, 37019);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 41097);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 43331);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 43332);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 41098);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 43333);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 43334);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 44342);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 44343);
									s.is_empty()
								}
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
			super::push(out, 37020);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -päismus.
pub(crate) fn n31819(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30449);
	s.is_empty()
}

/// Words ending -tsrecht.
pub(crate) fn n39473(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37330);
	match s {
		[] => true,
		['f', 'a', 'h', 'c', 's', 't', s @ ..] => {
			super::push(out, 41271);
			match s {
				[] => true,
				['r', 'i', s @ ..] => {
					super::push(out, 43428);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 41272);
			s.is_empty()
		}
		['ü', s @ ..] => {
			super::push(out, 41273);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -trecht.
pub(crate) fn n32242(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30823);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 37331);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 37332);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 37333);
			match s {
				[] => true,
				['e', 'w', 'm', 'u', '\u{1}', s @ ..] => {
					super::push(out, 41274);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 41275);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 41276);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zrecht.
pub(crate) fn n32244(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30825);
	s.is_empty()
}

/// Words ending -ßrecht.
pub(crate) fn n32245(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30826);
	match s {
		[] => true,
		['e', 'z', 'o', 'r', 'p', 'l', 'a', 'n', 'o', 'i', 't', 'a', 'n', 'r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
			super::push(out, 37334);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ischunterricht.
pub(crate) fn n46340(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 44421);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 44820);
					s.is_empty()
				}
				['r', 'o', 's', '\u{1}', s @ ..] => {
					super::push(out, 44821);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 44422);
			match s {
				[] => true,
				['c', 'e', 'h', s @ ..] => {
					super::push(out, 44822);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 44423);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 44424);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ikunterricht.
pub(crate) fn n43956(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 43469);
			match s {
				[] => true,
				['u', 'm', '\u{1}', s @ ..] => {
					super::push(out, 44425);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 44426);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'a', 'm', 'e', s @ ..] => {
			super::push(out, 43470);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lunterricht.
pub(crate) fn n43957(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 43471);
			s.is_empty()
		}
		['e', 'i', s @ ..] => {
			super::push(out, 43472);
			s.is_empty()
		}
		['u', 'h', 'c', 's', s @ ..] => {
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 44427);
					s.is_empty()
				}
				['s', 'f', s @ ..] => {
					super::push(out, 44428);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -echenunterricht.
pub(crate) fn n46346(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43473);
	s.is_empty()
}

/// Words ending -onunterricht.
pub(crate) fn n46347(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43474);
	s.is_empty()
}

/// Words ending -runterricht.
pub(crate) fn n43960(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41326);
	match s {
		[] => true,
		['e', 'd', s @ ..] => {
			super::push(out, 43475);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sunterricht.
pub(crate) fn n43961(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41327);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 43476);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 43477);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 43478);
			match s {
				[] => true,
				['o', 'i', 'g', s @ ..] => {
					super::push(out, 44429);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 43479);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 43480);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 43481);
			match s {
				[] => true,
				['a', 'h', '\u{1}', s @ ..] => {
					super::push(out, 44430);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 44431);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rtunterricht.
pub(crate) fn n43962(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41328);
	s.is_empty()
}

/// Words ending -zunterricht.
pub(crate) fn n43963(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41329);
	s.is_empty()
}

/// Words ending -tüberlegenheit.
pub(crate) fn n48032(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44972);
	s.is_empty()
}

/// Words ending -ossenheit.
pub(crate) fn n46393(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['l', 'h', 'c', 's', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 44825);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 44973);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 44826);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 44464);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -stenheit.
pub(crate) fn n39606(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37443);
	s.is_empty()
}

/// Words ending -hnheit.
pub(crate) fn n32363(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30929);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 37444);
			s.is_empty()
		}
		['ü', 'k', 'l', s @ ..] => {
			super::push(out, 37445);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -inheit.
pub(crate) fn n32364(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30930);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['n', 'e', 'm', s @ ..] => {
					super::push(out, 41391);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 41392);
					match s {
						[] => true,
						['e', 's', s @ ..] => {
							super::push(out, 43509);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 43510);
							s.is_empty()
						}
						['n', 'e', s @ ..] => {
							super::push(out, 43511);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 41393);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 41394);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 43512);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rnheit.
pub(crate) fn n32365(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30931);
	s.is_empty()
}

/// Words ending -önheit.
pub(crate) fn n32366(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30932);
	s.is_empty()
}

/// Words ending -hligkeit.
pub(crate) fn n39653(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37478);
	match s {
		[] => true,
		['ü', 'f', 'r', s @ ..] => {
			super::push(out, 41433);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -iligkeit.
pub(crate) fn n39654(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37479);
	s.is_empty()
}

/// Words ending -lligkeit.
pub(crate) fn n39655(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 41434);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 43530);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 41435);
			match s {
				[] => true,
				['w', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 44477);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 44478);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ä', 'f', s @ ..] => {
			match s {
				[] => true,
				['e', 'g', 't', s @ ..] => {
					super::push(out, 43531);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 44479);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 44480);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 44481);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'a', s @ ..] => {
					super::push(out, 43532);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mmigkeit.
pub(crate) fn n32403(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 37480);
			s.is_empty()
		}
		['ö', s @ ..] => {
			super::push(out, 37481);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nigkeit.
pub(crate) fn n32404(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30961);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 37482);
			s.is_empty()
		}
		['i', 'e', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 41436);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 41437);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 41438);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 41439);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 37483);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 41440);
					s.is_empty()
				}
				['ü', s @ ..] => {
					super::push(out, 41441);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 37484);
			s.is_empty()
		}
		['ö', s @ ..] => {
			super::push(out, 37485);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rigkeit.
pub(crate) fn n32405(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 37486);
			match s {
				[] => true,
				['i', 'w', 's', 't', s @ ..] => {
					super::push(out, 41442);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 37487);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 41443);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 37488);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 37489);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 37490);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 37491);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 41444);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 37492);
			s.is_empty()
		}
		['ö', 'h', 'e', 'g', s @ ..] => {
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', 's', s @ ..] => {
							super::push(out, 43533);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 43534);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 41445);
					match s {
						[] => true,
						['z', 'i', s @ ..] => {
							super::push(out, 43535);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sigkeit.
pub(crate) fn n32406(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30962);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 37493);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 37494);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['h', 'u', s @ ..] => {
									super::push(out, 44482);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 44483);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							match s {
								[] => true,
								['a', 'l', s @ ..] => {
									super::push(out, 44484);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 44485);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 44486);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							match s {
								[] => true,
								['e', 'r', s @ ..] => {
									super::push(out, 44487);
									s.is_empty()
								}
								['n', 'a', s @ ..] => {
									match s {
										[] => true,
										['h', s @ ..] => {
											super::push(out, 44827);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 44828);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'o', s @ ..] => {
									super::push(out, 44488);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', 'c', 'a', 'd', s @ ..] => {
							super::push(out, 43536);
							s.is_empty()
						}
						['k', 'c', 'a', s @ ..] => {
							super::push(out, 43537);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['g', 'ü', s @ ..] => {
											super::push(out, 44829);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 44830);
											s.is_empty()
										}
										['p', s @ ..] => {
											super::push(out, 44831);
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', s @ ..] => {
									super::push(out, 44489);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 43538);
							match s {
								[] => true,
								['r', 'o', s @ ..] => {
									super::push(out, 44490);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'e', s @ ..] => {
							match s {
								[] => true,
								['k', 'n', 'a', 'd', s @ ..] => {
									super::push(out, 44491);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 44492);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 44832);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 44833);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 44834);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 44493);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 44835);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 44836);
											s.is_empty()
										}
										_ => false,
									}
								}
								['g', 'n', 'u', s @ ..] => {
									match s {
										[] => true,
										['m', 'r', s @ ..] => {
											super::push(out, 44837);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 44838);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 44839);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 44494);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 44495);
									s.is_empty()
								}
								['s', 'u', s @ ..] => {
									super::push(out, 44496);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 44497);
									match s {
										[] => true,
										['f', 'a', 'h', 'c', 's', 'r', s @ ..] => {
											super::push(out, 44840);
											s.is_empty()
										}
										['h', 'c', s @ ..] => {
											match s {
												[] => true,
												['i', 's', 'b', s @ ..] => {
													super::push(out, 44974);
													s.is_empty()
												}
												['r', s @ ..] => {
													super::push(out, 44975);
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', s @ ..] => {
											super::push(out, 44841);
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
								['f', s @ ..] => super::t16::n47498(s, out),
								['h', 'c', s @ ..] => super::t16::n47499(s, out),
								['i', s @ ..] => super::t16::n47500(s, out),
								['k', s @ ..] => super::t16::n47501(s, out),
								['l', s @ ..] => super::t16::n47502(s, out),
								['s', s @ ..] => super::t16::n47503(s, out),
								['t', s @ ..] => super::t16::n47504(s, out),
								['u', s @ ..] => super::t16::n47505(s, out),
								['ä', s @ ..] => super::t16::n47506(s, out),
								_ => false,
							}
						}
						['z', s @ ..] => super::t16::n46437(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => super::t16::n39673(s, out),
		_ => false,
	}
}

/// Words ending -tigkeit.
pub(crate) fn n32407(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 37496);
			match s {
				[] => true,
				['a', 'h', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 44509);
									s.is_empty()
								}
								['n', 'e', s @ ..] => {
									super::push(out, 44510);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 43541);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 43542);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['f', s @ ..] => {
											super::push(out, 44846);
											s.is_empty()
										}
										['s', 'e', s @ ..] => {
											super::push(out, 44847);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 43543);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 44848);
											s.is_empty()
										}
										['r', 'h', 'e', 'n', s @ ..] => {
											super::push(out, 44849);
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
								['e', 't', s @ ..] => {
									super::push(out, 44511);
									s.is_empty()
								}
								['h', 'e', s @ ..] => {
									super::push(out, 44512);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'o', s @ ..] => {
							super::push(out, 43544);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 41449);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 41450);
					s.is_empty()
				}
				['r', 'ü', 'd', 'e', 'b', 'e', 'g', s @ ..] => {
					super::push(out, 41451);
					s.is_empty()
				}
				['ä', s @ ..] => {
					super::push(out, 41452);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 37497);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 43545);
							match s {
								[] => true,
								['r', 'e', 'g', s @ ..] => {
									match s {
										[] => true,
										['h', 'c', 'a', s @ ..] => {
											super::push(out, 44850);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 44851);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 44852);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 44853);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 43546);
							match s {
								[] => true,
								['d', 'r', s @ ..] => {
									super::push(out, 44513);
									s.is_empty()
								}
								['e', 'l', s @ ..] => {
									super::push(out, 44514);
									s.is_empty()
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 44854);
											s.is_empty()
										}
										['f', s @ ..] => {
											super::push(out, 44855);
											s.is_empty()
										}
										['h', s @ ..] => {
											super::push(out, 44856);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['f', s @ ..] => {
											super::push(out, 44857);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 44858);
											s.is_empty()
										}
										['r', s @ ..] => {
											match s {
												[] => true,
												['o', 'v', 'r', s @ ..] => {
													super::push(out, 44976);
													s.is_empty()
												}
												['ü', s @ ..] => {
													super::push(out, 44977);
													s.is_empty()
												}
												_ => false,
											}
										}
										['z', s @ ..] => {
											super::push(out, 44859);
											s.is_empty()
										}
										_ => false,
									}
								}
								['w', s @ ..] => {
									super::push(out, 44515);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ü', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 44516);
									s.is_empty()
								}
								['t', 'e', s @ ..] => {
									super::push(out, 44517);
									s.is_empty()
								}
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
			super::push(out, 37498);
			match s {
				[] => true,
				['e', 's', 'l', 'e', 'i', s @ ..] => {
					super::push(out, 41453);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 37499);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 41454);
					match s {
						[] => true,
						['f', 'g', s @ ..] => {
							super::push(out, 43547);
							s.is_empty()
						}
						['h', 't', s @ ..] => {
							super::push(out, 43548);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', 'f', 'l', s @ ..] => {
					super::push(out, 41455);
					s.is_empty()
				}
				['ü', s @ ..] => {
					super::push(out, 41456);
					match s {
						[] => true,
						['g', 'n', 'u', s @ ..] => {
							super::push(out, 43549);
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
				['a', s @ ..] => {
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 43550);
							s.is_empty()
						}
						['ß', s @ ..] => {
							super::push(out, 43551);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['f', 'd', s @ ..] => {
							super::push(out, 43552);
							s.is_empty()
						}
						['w', s @ ..] => {
							match s {
								[] => true,
								['i', 'e', 'r', s @ ..] => {
									super::push(out, 44518);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 44519);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ä', 'w', 'n', s @ ..] => {
					super::push(out, 41457);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['i', 'e', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 43553);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 43554);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 41458);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', 'a', s @ ..] => {
			super::push(out, 37500);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 37501);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 41459);
					s.is_empty()
				}
				['ä', s @ ..] => {
					super::push(out, 41460);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 37502);
			s.is_empty()
		}
		['ü', s @ ..] => {
			super::push(out, 37503);
			match s {
				[] => true,
				['m', 'r', s @ ..] => {
					super::push(out, 41461);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uigkeit.
pub(crate) fn n32408(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30963);
	match s {
		[] => true,
		['a', 'n', 'e', 'g', s @ ..] => {
			match s {
				[] => true,
				['l', 'i', s @ ..] => {
					super::push(out, 41462);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 41463);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zigkeit.
pub(crate) fn n32409(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 37504);
			s.is_empty()
		}
		['r', 'e', 'h', 'm', s @ ..] => {
			super::push(out, 37505);
			s.is_empty()
		}
		['t', 'o', s @ ..] => {
			super::push(out, 37506);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ßigkeit.
pub(crate) fn n32410(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30964);
	match s {
		[] => true,
		['ä', s @ ..] => {
			super::push(out, 37507);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['n', 'e', s @ ..] => {
							super::push(out, 43555);
							s.is_empty()
						}
						['t', 'o', s @ ..] => {
							super::push(out, 43556);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			super::push(out, 37508);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 41464);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 41465);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 41466);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -enfeindlichkeit.
pub(crate) fn n48075(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 45018);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 45019);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ofeindlichkeit.
pub(crate) fn n48076(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 45013);
	s.is_empty()
}

/// Words ending -änderfeindlichkeit.
pub(crate) fn n48077(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 45014);
	s.is_empty()
}

/// Words ending -echlichkeit.
pub(crate) fn n47534(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 44866);
			s.is_empty()
		}
		['t', 's', 'e', 'b', 'n', s @ ..] => {
			super::push(out, 44867);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ächlichkeit.
pub(crate) fn n47537(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 44868);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 44869);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -öhlichkeit.
pub(crate) fn n46473(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43568);
	s.is_empty()
}

/// Words ending -rparteilichkeit.
pub(crate) fn n44133(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41470);
	s.is_empty()
}

/// Words ending -klichkeit.
pub(crate) fn n44134(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41471);
	match s {
		[] => true,
		['c', 'ü', 'r', s @ ..] => {
			super::push(out, 43569);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mlichkeit.
pub(crate) fn n44135(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41472);
	match s {
		[] => true,
		['ü', s @ ..] => {
			super::push(out, 43570);
			match s {
				[] => true,
				['t', 's', s @ ..] => {
					super::push(out, 44528);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nlichkeit.
pub(crate) fn n44136(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41473);
	match s {
		[] => true,
		['h', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 44529);
					s.is_empty()
				}
				['ä', 'n', s @ ..] => {
					super::push(out, 44530);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 43571);
			match s {
				[] => true,
				['e', 'r', s @ ..] => {
					super::push(out, 44531);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 43572);
			match s {
				[] => true,
				['ä', s @ ..] => {
					super::push(out, 44532);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rlichkeit.
pub(crate) fn n44137(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41474);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['d', 'i', s @ ..] => {
					super::push(out, 44533);
					s.is_empty()
				}
				['g', 'r', s @ ..] => {
					super::push(out, 44534);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 44535);
					s.is_empty()
				}
				['i', 'e', s @ ..] => {
					super::push(out, 44536);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 44537);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 44538);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 44539);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 44540);
					s.is_empty()
				}
				['ß', s @ ..] => {
					super::push(out, 44541);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 44870);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 44871);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 44872);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', 'f', s @ ..] => {
					super::push(out, 44542);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 44873);
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
				['a', s @ ..] => {
					super::push(out, 44543);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 44544);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ü', 'k', s @ ..] => {
			super::push(out, 43573);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -slichkeit.
pub(crate) fn n44138(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 43574);
			s.is_empty()
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 44545);
					s.is_empty()
				}
				['ä', 'l', 'r', 'e', 'n', s @ ..] => {
					super::push(out, 44546);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'ä', s @ ..] => {
			super::push(out, 43575);
			s.is_empty()
		}
		['ö', 'l', s @ ..] => {
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 44547);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 44548);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tlichkeit.
pub(crate) fn n44139(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41475);
	match s {
		[] => true,
		['f', 'a', 'h', 'c', 's', s @ ..] => {
			match s {
				[] => true,
				['d', 'a', s @ ..] => {
					super::push(out, 44549);
					s.is_empty()
				}
				['n', 'e', 's', s @ ..] => {
					super::push(out, 44550);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 43576);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['l', 'h', 'c', 's', 'e', 'g', 's', s @ ..] => {
									super::push(out, 44978);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 44979);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 's', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 44980);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 44981);
									s.is_empty()
								}
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
			super::push(out, 43577);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 43578);
			s.is_empty()
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 44551);
					s.is_empty()
				}
				['n', 'e', 'k', 'r', s @ ..] => {
					super::push(out, 44552);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 43579);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 44553);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 43580);
			s.is_empty()
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['i', 'b', s @ ..] => {
					super::push(out, 44554);
					s.is_empty()
				}
				['ö', s @ ..] => {
					super::push(out, 44555);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 43581);
			s.is_empty()
		}
		['ä', s @ ..] => {
			super::push(out, 43582);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ulichkeit.
pub(crate) fn n44140(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['h', 'c', 's', 'e', s @ ..] => {
					super::push(out, 44556);
					s.is_empty()
				}
				['r', 'f', s @ ..] => {
					super::push(out, 44557);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 43583);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tzlichkeit.
pub(crate) fn n44141(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 43584);
			s.is_empty()
		}
		['ö', s @ ..] => {
			super::push(out, 43585);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nausschließlichkeit.
pub(crate) fn n44142(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41476);
	s.is_empty()
}

/// Words ending -ischkeit.
pub(crate) fn n39691(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37509);
	s.is_empty()
}

/// Words ending -peraktivität.
pub(crate) fn n48048(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44987);
	s.is_empty()
}

/// Words ending -teraktivität.
pub(crate) fn n48049(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44988);
	s.is_empty()
}

/// Words ending -objektivität.
pub(crate) fn n48050(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44989);
	s.is_empty()
}

/// Words ending -ubjektivität.
pub(crate) fn n48051(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44990);
	s.is_empty()
}

/// Words ending -ftlosigkeit.
pub(crate) fn n47498(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44498);
	s.is_empty()
}

/// Words ending -chtlosigkeit.
pub(crate) fn n47499(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 44842);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 44843);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -itlosigkeit.
pub(crate) fn n47500(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44499);
	s.is_empty()
}

/// Words ending -ktlosigkeit.
pub(crate) fn n47501(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44500);
	s.is_empty()
}

/// Words ending -ltlosigkeit.
pub(crate) fn n47502(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44501);
	s.is_empty()
}

/// Words ending -stlosigkeit.
pub(crate) fn n47503(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44502);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 44844);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ttlosigkeit.
pub(crate) fn n47504(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44503);
	s.is_empty()
}

/// Words ending -utlosigkeit.
pub(crate) fn n47505(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44504);
	s.is_empty()
}

/// Words ending -ätlosigkeit.
pub(crate) fn n47506(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44505);
	s.is_empty()
}

/// Words ending -zlosigkeit.
pub(crate) fn n46437(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 44506);
			s.is_empty()
		}
		['t', 'u', 'n', s @ ..] => {
			super::push(out, 44507);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ssigkeit.
pub(crate) fn n39673(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37495);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 41446);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 41447);
			s.is_empty()
		}
		['ä', 'l', 'r', 'h', 'a', 'f', 's', s @ ..] => {
			super::push(out, 41448);
			s.is_empty()
		}
		['ü', s @ ..] => {
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 43539);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 43540);
					match s {
						[] => true,
						['f', 'h', s @ ..] => {
							super::push(out, 44508);
							match s {
								[] => true,
								['ä', s @ ..] => {
									super::push(out, 44845);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

