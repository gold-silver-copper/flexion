use flexion::Path;

/// Words ending -вые.
pub(crate) fn n1611(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1607);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 8650);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27221);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 54624);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 27222);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 54625);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 82855);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 105488);
											match s {
												[] => true,
												['ь', s @ ..] => {
													super::push(out, 120943);
													match s {
														[] => true,
														['л', s @ ..] => super::t33::n131815(s, out),
														_ => false,
													}
												}
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
							super::push(out, 54626);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 82856);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 105489);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 120944);
													match s {
														[] => true,
														['а', s @ ..] => super::t33::n131816(s, out),
														_ => false,
													}
												}
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
							super::push(out, 54627);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 82857);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 105490);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 120945);
													match s {
														[] => true,
														['ф', s @ ..] => super::t33::n131817(s, out),
														_ => false,
													}
												}
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
							super::push(out, 54628);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 82858);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 105491);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 120946);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 82859);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 105492);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 54629);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 82860);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 105493);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 120947);
													match s {
														[] => true,
														['б', s @ ..] => super::t33::n131818(s, out),
														_ => false,
													}
												}
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
							super::push(out, 54630);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 82861);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 105494);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 120948);
													match s {
														[] => true,
														['л', s @ ..] => super::t33::n131819(s, out),
														_ => false,
													}
												}
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
							super::push(out, 54631);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 82862);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 105495);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 120949);
													match s {
														[] => true,
														['к', s @ ..] => super::t33::n131820(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ф', s @ ..] => {
											super::push(out, 105496);
											match s {
												[] => true,
												['о', s @ ..] => super::t33::n122221(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => super::t33::n27410(s, out),
				['о', s @ ..] => {
					super::push(out, 27224);
					match s {
						[] => true,
						['б', s @ ..] => super::t33::n55192(s, out),
						_ => false,
					}
				}
				['р', s @ ..] => super::t33::n27412(s, out),
				['с', s @ ..] => super::t33::n27413(s, out),
				['ч', s @ ..] => super::t33::n27414(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t33::n8677(s, out),
		['ю', s @ ..] => super::t33::n8678(s, out),
		_ => false,
	}
}

/// Words ending -рно-белые.
pub(crate) fn n122299(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121028);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 130518);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 136071);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -овокрылые.
pub(crate) fn n122300(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121029);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 130519);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -укокрылые.
pub(crate) fn n122301(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121030);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 130520);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ннокрылые.
pub(crate) fn n122302(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121031);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 130521);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 136072);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -двукрылые.
pub(crate) fn n122303(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121032);
	s.is_empty()
}

/// Words ending -ные.
pub(crate) fn n1614(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1610);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 8656);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 27246);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 54694);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 82947);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 105590);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8657);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 27247);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 54695);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 82948);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 105591);
											match s {
												[] => true,
												['ц', s @ ..] => {
													super::push(out, 121033);
													match s {
														[] => true,
														['с', s @ ..] => super::t33::n131878(s, out),
														_ => false,
													}
												}
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
							super::push(out, 54696);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 82949);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 105592);
											s.is_empty()
										}
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
					super::push(out, 27248);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 54697);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 54698);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 82950);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 105593);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 121034);
													match s {
														[] => true,
														['о', s @ ..] => super::t33::n131879(s, out),
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
									super::push(out, 82951);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 105594);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8658);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 27249);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 54699);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 82952);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 105595);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 121035);
													match s {
														[] => true,
														['н', s @ ..] => super::t33::n131880(s, out),
														['т', s @ ..] => super::t33::n131881(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 8659);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27250);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 54700);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 82953);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 105596);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 54701);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 82954);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 105597);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 121036);
													match s {
														[] => true,
														['п', s @ ..] => super::t33::n131882(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => super::t33::n8686(s, out),
		['и', s @ ..] => {
			super::push(out, 8661);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 27252);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 54703);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 82955);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 105600);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 121044);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 27253);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 54704);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 82956);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 27254);
					match s {
						[] => true,
						['б', s @ ..] => super::t33::n55264(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => super::t33::n8688(s, out),
		['н', s @ ..] => super::t33::n8689(s, out),
		['п', s @ ..] => super::t33::n8690(s, out),
		['р', s @ ..] => super::t33::n8691(s, out),
		['с', s @ ..] => super::t33::n8692(s, out),
		['т', s @ ..] => super::t33::n8693(s, out),
		['ч', s @ ..] => super::t33::n8694(s, out),
		['ь', s @ ..] => super::t33::n8695(s, out),
		_ => false,
	}
}

/// Words ending -степерые.
pub(crate) fn n106754(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 105630);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 121076);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 130564);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чеперые.
pub(crate) fn n83862(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82987);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 105631);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 121077);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сые.
pub(crate) fn n1616(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1612);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 8671);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 27275);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 54731);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 82988);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 27276);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 54732);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 82989);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тые.
pub(crate) fn n1617(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1613);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8672);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 27277);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 54733);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 82990);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 105632);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 121078);
													match s {
														[] => true,
														['ч', s @ ..] => super::t33::n131922(s, out),
														_ => false,
													}
												}
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
							super::push(out, 54734);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 82991);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 105633);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 121079);
													match s {
														[] => true,
														['б', s @ ..] => super::t33::n131923(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 8673);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 27278);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 54735);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 82992);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 105634);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 121080);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 8674);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 27279);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 54736);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 82993);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 105635);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 121081);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 8675);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27280);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 54737);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 82994);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 105636);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 121082);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рупнотравье.
pub(crate) fn n122355(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121083);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 130567);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -овье.
pub(crate) fn n8711(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8685);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 27298);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 54750);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 83004);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 105640);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 121084);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 27299);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 27300);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 54751);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 83005);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 54752);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 54753);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 83006);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 105641);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 54754);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 83007);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 105642);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 121085);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 130568);
													match s {
														[] => true,
														['п', s @ ..] => super::t33::n137514(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 27301);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 54755);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 27302);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 54756);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 83008);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 27303);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 54757);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 54758);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 54759);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 83009);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 27304);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 54760);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 83010);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 105643);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 54761);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 83011);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 105644);
									s.is_empty()
								}
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
			super::push(out, 27305);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 54762);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 27306);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 54763);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 83012);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 105645);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 121086);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 130569);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дье.
pub(crate) fn n1620(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1616);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8686);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 27307);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 27308);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 54764);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 83013);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 105646);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', 'г', 'х', 'е', 'р', 'т', '\u{1}', s @ ..] => {
					super::push(out, 27309);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8687);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 27310);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 54765);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 8688);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27311);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 54766);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 83014);
									s.is_empty()
								}
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
			super::push(out, 8689);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 27312);
					match s {
						[] => true,
						['з', 'а', s @ ..] => {
							super::push(out, 54767);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 27313);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 27314);
					match s {
						[] => true,
						['о', 'г', s @ ..] => {
							super::push(out, 54768);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 83015);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 105647);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 121087);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t33::n131927(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 8690);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 54769);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 54770);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 8691);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 27315);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жье.
pub(crate) fn n1621(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1617);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8692);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 8693);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 27316);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 54771);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 27317);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 54772);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 27318);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 54773);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 83016);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 105648);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 121088);
													match s {
														[] => true,
														['б', s @ ..] => super::t33::n131928(s, out),
														_ => false,
													}
												}
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
							super::push(out, 54774);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 83017);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 105649);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 54775);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 83018);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 105650);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 121089);
													s.is_empty()
												}
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
							super::push(out, 54776);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 83019);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 83020);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 105651);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 121090);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 27319);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 54777);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 54778);
							match s {
								[] => true,
								['б', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 105652);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 121091);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t33::n131929(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 105653);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8694);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27320);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8695);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 27321);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 27322);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 54779);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 27323);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 54780);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 83021);
									s.is_empty()
								}
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
			super::push(out, 8696);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 27324);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 27325);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 54781);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 54782);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 83022);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 105654);
											match s {
												[] => true,
												['ф', s @ ..] => {
													super::push(out, 121092);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 8697);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 27326);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 54783);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 83023);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 105655);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -азье.
pub(crate) fn n8724(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8698);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 27327);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 54784);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 83024);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 105656);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 121093);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 130573);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 27328);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 54785);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -безье.
pub(crate) fn n27517(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27329);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 54786);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кье.
pub(crate) fn n1623(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1619);
	match s {
		[] => true,
		['ю', s @ ..] => {
			super::push(out, 8700);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 27330);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 54787);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 83025);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лье.
pub(crate) fn n1624(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1620);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8701);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27331);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 27332);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 54788);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 83026);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 105657);
											s.is_empty()
										}
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
					super::push(out, 27333);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 54789);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 83027);
									s.is_empty()
								}
								_ => false,
							}
						}
						['й', s @ ..] => {
							super::push(out, 54790);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 54791);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 83028);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 27334);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8702);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 27335);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 27336);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 54792);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 54793);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 27337);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 27338);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 54794);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 83029);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 54795);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 83030);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 105658);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 54796);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 27339);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 54797);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 83031);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 105659);
											s.is_empty()
										}
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
					super::push(out, 27340);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 54798);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 83032);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 27341);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 54799);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 54800);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 27342);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 54801);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 83033);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 54802);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 83034);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 105660);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 121094);
													match s {
														[] => true,
														['с', s @ ..] => super::t33::n131931(s, out),
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
									super::push(out, 83035);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 105661);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 121095);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 27343);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 54803);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 83036);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 27344);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 54804);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 83037);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 105662);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 121096);
													s.is_empty()
												}
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
							super::push(out, 54805);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 8703);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 27345);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 54806);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 83038);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 105663);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 121097);
													s.is_empty()
												}
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
							super::push(out, 54807);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 27346);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 54808);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 83039);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 54809);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 83040);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 27347);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 54810);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 83041);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 105664);
											s.is_empty()
										}
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
					super::push(out, 27348);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 54811);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 27349);
					match s {
						[] => true,
						['а', s @ ..] => super::t33::n55371(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t33::n8730(s, out),
		['у', s @ ..] => super::t33::n8731(s, out),
		['ы', s @ ..] => super::t33::n8732(s, out),
		_ => false,
	}
}

/// Words ending -мье.
pub(crate) fn n1625(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1621);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8707);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 27364);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 54827);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 27365);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8708);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 8709);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 8710);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 27366);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 54828);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 8711);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 27367);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 54829);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 83052);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 105673);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8712);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нье.
pub(crate) fn n1626(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1622);
	match s {
		[] => true,
		['а', s @ ..] => super::t33::n8739(s, out),
		['е', s @ ..] => super::t33::n8740(s, out),
		['и', s @ ..] => {
			super::push(out, 8715);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 27402);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 54989);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 83306);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 105925);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 121281);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 8716);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 27403);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 54990);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 27404);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 54991);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 83307);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 27405);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 54992);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 27406);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 54993);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 83308);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 105926);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 121282);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t33::n132041(s, out),
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
									super::push(out, 83309);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 27407);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 54994);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 83310);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 27408);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 54995);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 83311);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 105927);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 121283);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 8717);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 27409);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 54996);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 83312);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 105928);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8718);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27410);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 54997);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 54998);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 54999);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 83313);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 55000);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 83314);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 55001);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 83315);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 83316);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 105929);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 55002);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 83317);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 55003);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 83318);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 105930);
											s.is_empty()
										}
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
					super::push(out, 27411);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 55004);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 83319);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 55005);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 83320);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 105931);
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 83321);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 105932);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 55006);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 55007);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 83322);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 55008);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 83323);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 83324);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 105933);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 121284);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 27412);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 27413);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 55009);
							match s {
								[] => true,
								['а', s @ ..] => super::t33::n84216(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => super::t33::n27602(s, out),
				['р', s @ ..] => super::t33::n27603(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пье.
pub(crate) fn n1627(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1623);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 8719);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 27416);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55012);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 27417);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 55013);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 83326);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 27418);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 55014);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 83327);
									s.is_empty()
								}
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
			super::push(out, 8720);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 27419);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 55015);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 83328);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 27420);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 55016);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 83329);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 55017);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 83330);
									s.is_empty()
								}
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
			super::push(out, 8721);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 27421);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 55018);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 83331);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 27422);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 8722);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 27423);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 55019);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рье.
pub(crate) fn n1628(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1624);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8723);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 27424);
					s.is_empty()
				}
				['в', s @ ..] => {
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 55020);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 83332);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 105935);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 55021);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 27425);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 27426);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 55022);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 83333);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 105936);
											s.is_empty()
										}
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
					super::push(out, 27427);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55023);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 83334);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 27428);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 27429);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 27430);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55024);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 83335);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 55025);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 27431);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8724);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 27432);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 27433);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 55026);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 55027);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 27434);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 27435);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 55028);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 83336);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 27436);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 55029);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 83337);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 105937);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 121286);
													match s {
														[] => true,
														['о', s @ ..] => super::t33::n132042(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 27437);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 55030);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 83338);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 105938);
											s.is_empty()
										}
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
					super::push(out, 27438);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 55031);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 8725);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 27439);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8726);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 27440);
					match s {
						[] => true,
						['х', 'е', 'р', 'ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
							super::push(out, 55032);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 27441);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 55033);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 27442);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 55034);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 83339);
									match s {
										[] => true,
										['я', s @ ..] => {
											super::push(out, 105939);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 121287);
													s.is_empty()
												}
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
							super::push(out, 55035);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 27443);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 55036);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 83340);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 27444);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 27445);
					match s {
						[] => true,
						['о', 'н', s @ ..] => {
							super::push(out, 55037);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 83341);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 105940);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 121288);
													match s {
														[] => true,
														['и', s @ ..] => super::t33::n132043(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 27446);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 55038);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 27447);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 8727);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 27448);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 55039);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 83342);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 105941);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 121289);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 27449);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => super::t33::n8754(s, out),
		['ю', s @ ..] => super::t33::n8755(s, out),
		['я', s @ ..] => {
			super::push(out, 8730);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 27452);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сье.
pub(crate) fn n1629(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1625);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8731);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 27453);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8732);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 27454);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 55044);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 83345);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 105944);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 121290);
													match s {
														[] => true,
														['п', s @ ..] => super::t33::n132044(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 27455);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55045);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 27456);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55046);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 8733);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 27457);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 55047);
							match s {
								[] => true,
								['к', '\u{1}', s @ ..] => {
									super::push(out, 83346);
									s.is_empty()
								}
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
			super::push(out, 8734);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27458);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 55048);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 55049);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 83347);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 105945);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 121291);
													s.is_empty()
												}
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
							super::push(out, 55050);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 83348);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 105946);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 121292);
													match s {
														[] => true,
														['о', s @ ..] => super::t33::n132045(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 8735);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 27459);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55051);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 27460);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55052);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 83349);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 27461);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55053);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 83350);
									s.is_empty()
								}
								['г', s @ ..] => {
									super::push(out, 83351);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 105947);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 105948);
											s.is_empty()
										}
										_ => false,
									}
								}
								['к', s @ ..] => {
									super::push(out, 83352);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 27462);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55054);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 27463);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 55055);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 83353);
									match s {
										[] => true,
										['е', 'п', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 121293);
													s.is_empty()
												}
												['д', s @ ..] => {
													super::push(out, 121294);
													match s {
														[] => true,
														['а', s @ ..] => super::t33::n132046(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 8736);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тье.
pub(crate) fn n1630(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1626);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8737);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 27464);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 55056);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 83354);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 105949);
											s.is_empty()
										}
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
					super::push(out, 27465);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55057);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 83355);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 27466);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55058);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 83356);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 105950);
											match s {
												[] => true,
												['ш', s @ ..] => {
													super::push(out, 121295);
													s.is_empty()
												}
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
							super::push(out, 55059);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 83357);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 105951);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 121296);
													match s {
														[] => true,
														['в', s @ ..] => super::t33::n132047(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 27467);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55060);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 27468);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 55061);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 27469);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 55062);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 83358);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 27470);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8738);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 27471);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 27472);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55063);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 55064);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 27473);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 55065);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 83359);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 55066);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 83360);
									s.is_empty()
								}
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
			super::push(out, 8739);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27474);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 27475);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55067);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', 'з', s @ ..] => {
					super::push(out, 27476);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 27477);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 55068);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 83361);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 105952);
											s.is_empty()
										}
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
					super::push(out, 27478);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 55069);
							s.is_empty()
						}
						['о', 'р', 'п', '\u{1}', s @ ..] => {
							super::push(out, 55070);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 55071);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 83362);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 105953);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 55072);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 83363);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 105954);
											s.is_empty()
										}
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
					super::push(out, 27479);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 27480);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 55073);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 55074);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 27481);
					match s {
						[] => true,
						['б', 'о', s @ ..] => {
							super::push(out, 55075);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 83364);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 27482);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 8740);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27483);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 55076);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 83365);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 105955);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 121297);
													s.is_empty()
												}
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
							super::push(out, 55077);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t33::n8767(s, out),
		['р', s @ ..] => {
			super::push(out, 8742);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 27489);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 55085);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 83375);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => super::t33::n27680(s, out),
				['у', s @ ..] => super::t33::n27681(s, out),
				_ => false,
			}
		}
		['с', s @ ..] => super::t33::n8769(s, out),
		['у', s @ ..] => super::t33::n8770(s, out),
		['ы', s @ ..] => super::t33::n8771(s, out),
		['ю', s @ ..] => super::t33::n8772(s, out),
		['я', s @ ..] => super::t33::n8773(s, out),
		_ => false,
	}
}

/// Words ending -фье.
pub(crate) fn n1631(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1627);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 8748);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 27514);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 55110);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 83391);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чье.
pub(crate) fn n1632(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1628);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8749);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 27515);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55111);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 83392);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 105966);
											s.is_empty()
										}
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
					super::push(out, 27516);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 55112);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 83393);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 27517);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 55113);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 83394);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 27518);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 55114);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 83395);
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
						['б', s @ ..] => {
							super::push(out, 55115);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 83396);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 105967);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 121303);
													s.is_empty()
												}
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
							super::push(out, 55116);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 83397);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 27519);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 55117);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 83398);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 105968);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 121304);
													match s {
														[] => true,
														['д', s @ ..] => super::t33::n132050(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 8750);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 27520);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 55118);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 83399);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 105969);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 121305);
													match s {
														[] => true,
														['д', s @ ..] => super::t33::n132051(s, out),
														_ => false,
													}
												}
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
							super::push(out, 55119);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 83400);
									s.is_empty()
								}
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
			super::push(out, 8751);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 27521);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55120);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 83401);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 105970);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 121306);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 27522);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 55121);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 83402);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 27523);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 27524);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 27525);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55122);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 83403);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 105971);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8752);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 27526);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55123);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 27527);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55124);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 83404);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 105972);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 121307);
													match s {
														[] => true,
														['п', s @ ..] => super::t33::n132052(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => super::t33::n27719(s, out),
				['н', s @ ..] => {
					super::push(out, 27529);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55127);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t33::n27721(s, out),
				_ => false,
			}
		}
		['у', s @ ..] => super::t33::n8779(s, out),
		['ы', s @ ..] => super::t33::n8780(s, out),
		_ => false,
	}
}

/// Words ending -шье.
pub(crate) fn n1633(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1629);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 8755);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 8756);
			match s {
				[] => true,
				['д', 'о', s @ ..] => {
					super::push(out, 27537);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 27538);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -километраж.
pub(crate) fn n132053(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130696);
	s.is_empty()
}

/// Words ending -охронометраж.
pub(crate) fn n132054(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130697);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 136173);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 139371);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 141221);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 142283);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -церт-вернисаж.
pub(crate) fn n132055(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130698);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 136174);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 139372);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -атье-корсаж.
pub(crate) fn n132056(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130699);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 136175);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 139373);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ан-репортаж.
pub(crate) fn n132057(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130700);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 136176);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 139374);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 141222);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рри-бридж.
pub(crate) fn n122590(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121316);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 130701);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 136177);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -керидж.
pub(crate) fn n55767(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 55195);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 83448);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 105997);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 121317);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -поридж.
pub(crate) fn n55768(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 55196);
	s.is_empty()
}

/// Words ending -тридж.
pub(crate) fn n27806(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27613);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 55197);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 83449);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 105998);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эридж.
pub(crate) fn n27807(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27614);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 55198);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -йдж.
pub(crate) fn n1653(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1649);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 8821);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 27615);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лдж.
pub(crate) fn n1654(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1650);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8822);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 27616);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ндж.
pub(crate) fn n1655(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1651);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8823);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 27617);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8824);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 27618);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 55199);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 83450);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 105999);
											s.is_empty()
										}
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
					super::push(out, 27619);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 55200);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 83451);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 106000);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 121318);
													match s {
														[] => true,
														['с', s @ ..] => super::t33::n132059(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 27620);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 8825);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 27621);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 55201);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 8826);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27622);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 55202);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 83452);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рдж.
pub(crate) fn n1656(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1652);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8827);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 27623);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55203);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8828);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 27624);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 8829);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 27625);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 55204);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 83453);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -удж.
pub(crate) fn n1657(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1653);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 8830);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 27626);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 55205);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эдж.
pub(crate) fn n1658(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1654);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 8831);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 8832);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -бульденеж.
pub(crate) fn n122593(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121319);
	s.is_empty()
}

/// The word деунеж.
pub(crate) fn n84354(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 83462);
	s.is_empty()
}

/// Words ending -пеж.
pub(crate) fn n1667(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1663);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8858);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 27650);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55221);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 8859);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 27651);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 8860);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 27652);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55222);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 27653);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55223);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 8861);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 27654);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 55224);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -реж.
pub(crate) fn n1668(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1664);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 8862);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 8863);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 27655);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 8864);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 27656);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 55225);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 8865);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27657);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 55226);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 83463);
									s.is_empty()
								}
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
			super::push(out, 8866);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 27658);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 55227);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 83464);
									match s {
										[] => true,
										['ы', s @ ..] => {
											super::push(out, 106004);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8867);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27659);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 55228);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 83465);
									s.is_empty()
								}
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
			super::push(out, 8868);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 27660);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 55229);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 83466);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сеж.
pub(crate) fn n1669(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1665);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 8869);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -теж.
pub(crate) fn n1670(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1666);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 8870);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 27661);
					match s {
						[] => true,
						['п', 'о', s @ ..] => {
							super::push(out, 55230);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 83467);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106005);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 8871);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27662);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 27663);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 55231);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 27664);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 55232);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 8872);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 27665);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 27666);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 55233);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 83468);
									s.is_empty()
								}
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
			super::push(out, 8873);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 27667);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чеж.
pub(crate) fn n1671(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1667);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 8874);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 27668);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 55234);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 83469);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьеж.
pub(crate) fn n1672(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1668);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 8875);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27669);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -елесторож.
pub(crate) fn n122594(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121320);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 130703);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ланк-заказ.
pub(crate) fn n132061(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130704);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 136178);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -упон-заказ.
pub(crate) fn n132062(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130705);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 136179);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кция-показ.
pub(crate) fn n132063(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130706);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 136180);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 139375);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -глаз-алмаз.
pub(crate) fn n132064(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130707);
	s.is_empty()
}

/// The word сингалез.
pub(crate) fn n122600(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121326);
	s.is_empty()
}

/// Words ending -абиогенез.
pub(crate) fn n122601(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121327);
	s.is_empty()
}

/// Words ending -ерез.
pub(crate) fn n9031(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 27774);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 55310);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 83521);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 106028);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 121328);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 27775);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 55311);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word крез.
pub(crate) fn n27970(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27776);
	s.is_empty()
}

/// Words ending -орез.
pub(crate) fn n9034(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9007);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 27777);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 55312);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 83522);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 106029);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 121329);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', 'з', s @ ..] => {
					super::push(out, 55313);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 83523);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 27778);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 55314);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 83524);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 106030);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 55315);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 83525);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 106031);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 121330);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 27779);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 27780);
			s.is_empty()
		}
		['л', 'к', s @ ..] => {
			super::push(out, 27781);
			match s {
				[] => true,
				['е', 'т', 'с', '\u{1}', s @ ..] => {
					super::push(out, 55316);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 27782);
			s.is_empty()
		}
		['п', 'и', s @ ..] => {
			super::push(out, 27783);
			match s {
				[] => true,
				['ш', '\u{1}', s @ ..] => {
					super::push(out, 55317);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 27784);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 27785);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 55318);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 55319);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 83526);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 106032);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 121331);
											s.is_empty()
										}
										['-', s @ ..] => {
											super::push(out, 121332);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 130708);
													match s {
														[] => true,
														['л', s @ ..] => super::t33::n137592(s, out),
														_ => false,
													}
												}
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
							super::push(out, 83527);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 106033);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 121333);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 83528);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 27786);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -през.
pub(crate) fn n9035(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9008);
	s.is_empty()
}

/// Words ending -урез.
pub(crate) fn n9036(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9009);
	s.is_empty()
}

/// Words ending -ырез.
pub(crate) fn n9037(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9010);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 27787);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 55320);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тез.
pub(crate) fn n1740(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1736);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 9011);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27788);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 55321);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 83529);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 106034);
											s.is_empty()
										}
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
					super::push(out, 27789);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 55322);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 83530);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 106035);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 121334);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 27790);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 55323);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 83531);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 106036);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 121335);
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 106037);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9012);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 27791);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 55324);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 9013);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 27792);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 55325);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьез.
pub(crate) fn n1741(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1737);
	s.is_empty()
}

/// The word зализ.
pub(crate) fn n55913(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 55340);
	s.is_empty()
}

/// Words ending -кализ.
pub(crate) fn n28005(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27811);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 55341);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 83539);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 106040);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нализ.
pub(crate) fn n28006(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27812);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 55342);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 83540);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 83541);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 106041);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 121336);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тализ.
pub(crate) fn n28007(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27813);
	s.is_empty()
}

/// Words ending -елиз.
pub(crate) fn n9059(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9032);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 27814);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 27815);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 55343);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -клиз.
pub(crate) fn n9060(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9033);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 27816);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -олиз.
pub(crate) fn n9061(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9034);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 27817);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 55344);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 83542);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 106042);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 121337);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 55345);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 83543);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 106043);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 121338);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 27818);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 27819);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 27820);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 55346);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 83544);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 106044);
									s.is_empty()
								}
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
			super::push(out, 27821);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 27822);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рлиз.
pub(crate) fn n9062(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9035);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 27823);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 55347);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 83545);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -миз.
pub(crate) fn n1751(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1747);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9036);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 27824);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 9037);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 27825);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 9038);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 27826);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 55348);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 9039);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 27827);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 55349);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 9040);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27828);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 55350);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -низ.
pub(crate) fn n1752(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1748);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9041);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 9042);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 27829);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55351);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 9043);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27830);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 55352);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 27831);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -риз.
pub(crate) fn n1753(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1749);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9044);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 9045);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 27832);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55353);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 9046);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 9047);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 27833);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 55354);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 83546);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 106045);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 121339);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 9048);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 27834);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 55355);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 83547);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 106046);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 121340);
													match s {
														[] => true,
														['к', s @ ..] => super::t33::n132066(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 9049);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 9050);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27835);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 27836);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 55356);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 27837);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 55357);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 83548);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 106047);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 121341);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ю', s @ ..] => {
							super::push(out, 55358);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 83549);
									s.is_empty()
								}
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
			super::push(out, 9051);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27838);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 27839);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 55359);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 83550);
									s.is_empty()
								}
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
			super::push(out, 9052);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 27840);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тиз.
pub(crate) fn n1754(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1750);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9053);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 27841);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55360);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 83551);
									s.is_empty()
								}
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
			super::push(out, 9054);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 27842);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 27843);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 9055);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 27844);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 9056);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 27845);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 55361);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 83552);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уиз.
pub(crate) fn n1755(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1751);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 9057);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 27846);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -физ.
pub(crate) fn n1756(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1752);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9058);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 27847);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 55362);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 27848);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55363);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 9059);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 27849);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 55364);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 9060);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 27850);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 55365);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 9061);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 27851);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 9062);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 27852);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 55366);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -циз.
pub(crate) fn n1757(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1753);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 9063);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27853);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шиз.
pub(crate) fn n1758(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1754);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9064);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 9065);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27854);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 55367);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -омо-герлз.
pub(crate) fn n122616(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121342);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 130710);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ь-молоковоз.
pub(crate) fn n122617(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121343);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 130711);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 136182);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 139377);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 141224);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тяжеловоз.
pub(crate) fn n122618(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121344);
	s.is_empty()
}

/// Words ending -умовоз.
pub(crate) fn n55949(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 55376);
	match s {
		[] => true,
		['т', 'и', 'б', '-', s @ ..] => {
			super::push(out, 83558);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 106052);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 121345);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 130712);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 136183);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 139378);
													match s {
														[] => true,
														['м', s @ ..] => super::t33::n142671(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еновоз.
pub(crate) fn n55950(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 55377);
	s.is_empty()
}

/// Words ending -пновоз.
pub(crate) fn n55951(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 55378);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 83559);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 106053);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 121346);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ровоз.
pub(crate) fn n28063(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27869);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 55379);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 83560);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 106054);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 121347);
									s.is_empty()
								}
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
			super::push(out, 55380);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 55381);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 83561);
					match s {
						[] => true,
						['у', 'м', '-', s @ ..] => {
							super::push(out, 106055);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 121348);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 130713);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 136184);
													match s {
														[] => true,
														['б', s @ ..] => super::t33::n140819(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 55382);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -товоз.
pub(crate) fn n28064(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27870);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 55383);
			match s {
				[] => true,
				['е', 'м', 'е', s @ ..] => {
					super::push(out, 83562);
					match s {
						[] => true,
						['ц', '-', s @ ..] => {
							super::push(out, 106056);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 121349);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 130714);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 136185);
													match s {
														[] => true,
														['б', s @ ..] => super::t33::n140820(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ховоз.
pub(crate) fn n28065(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27871);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 55384);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 83563);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 106057);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 121350);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ывоз.
pub(crate) fn n9103(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9076);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 27872);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 55385);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 55386);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 83564);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 106058);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 83565);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гоз.
pub(crate) fn n1765(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1761);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9077);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 27873);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 9078);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -доз.
pub(crate) fn n1766(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1762);
	s.is_empty()
}

/// Words ending -иоз.
pub(crate) fn n1767(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1763);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 9079);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 27874);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 55387);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 83566);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106059);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -коз.
pub(crate) fn n1768(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1764);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9080);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 27875);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 9081);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 9082);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27876);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 55388);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 83567);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 106060);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 121351);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лоз.
pub(crate) fn n1769(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1765);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 9083);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 9084);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 27877);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 27878);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55389);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -моз.
pub(crate) fn n1770(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1766);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 9085);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 27879);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55390);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 9086);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 9087);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27880);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 55391);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 83568);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ноз.
pub(crate) fn n1771(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1767);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 9088);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27881);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 55392);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 83569);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 27882);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 55393);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 83570);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 83571);
									s.is_empty()
								}
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
			super::push(out, 9089);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 9090);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 27883);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 55394);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 83572);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 106061);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 121352);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 9091);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27884);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 55395);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 83573);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 106062);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 121353);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 9092);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 9093);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -роз.
pub(crate) fn n1772(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1768);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 9094);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 9095);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 27885);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55396);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 55397);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 83574);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 106063);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 121354);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 27886);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 55398);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 83575);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 27887);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55399);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 9096);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тоз.
pub(crate) fn n1773(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1769);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9097);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 27888);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 27889);
					match s {
						[] => true,
						['е', 'к', 'р', s @ ..] => {
							super::push(out, 55400);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 83576);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 106064);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 121355);
													match s {
														[] => true,
														['г', s @ ..] => super::t33::n132072(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 9098);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уоз.
pub(crate) fn n1774(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1770);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 9099);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 27890);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 55401);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 83577);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фоз.
pub(crate) fn n1775(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1771);
	s.is_empty()
}

/// Words ending -хоз.
pub(crate) fn n1776(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1772);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 9100);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27891);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 55402);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 83578);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 27892);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 55403);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 9101);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 9102);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27893);
					match s {
						[] => true,
						['к', '-', s @ ..] => {
							super::push(out, 55404);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 83579);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 106065);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 121356);
													match s {
														[] => true,
														['и', s @ ..] => super::t33::n132073(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 9103);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 9104);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 27894);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 55405);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 83580);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 106066);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 121357);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 9105);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27895);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 55406);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 83581);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ерциргауз.
pub(crate) fn n122632(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121358);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 130717);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 136187);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 139382);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -втуз.
pub(crate) fn n9158(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9131);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 27919);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 55424);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нтуз.
pub(crate) fn n9159(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9132);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 27920);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 55425);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 83593);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 55426);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 83594);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ртуз.
pub(crate) fn n9160(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9133);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 27921);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 55427);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -утуз.
pub(crate) fn n9161(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9134);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 27922);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 55428);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фуз.
pub(crate) fn n1796(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1792);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 9135);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 27923);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хуз.
pub(crate) fn n1797(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1793);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 9136);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 27924);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 55429);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цуз.
pub(crate) fn n1798(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1794);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 9137);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 27925);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 55430);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 83595);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word главкоюз.
pub(crate) fn n122633(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121359);
	s.is_empty()
}

/// Words ending -ень-на-оби.
pub(crate) fn n132075(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130718);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 136188);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -дог-фрисби.
pub(crate) fn n132076(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130719);
	s.is_empty()
}

/// Words ending --энд-хеви.
pub(crate) fn n122636(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121362);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 130720);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -киви-киви.
pub(crate) fn n122637(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121363);
	s.is_empty()
}

/// Words ending -роуд-муви.
pub(crate) fn n122638(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121364);
	s.is_empty()
}

/// Words ending -ахви.
pub(crate) fn n9263(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9236);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 28020);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 55489);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыхви.
pub(crate) fn n9264(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9237);
	match s {
		[] => true,
		['й', s @ ..] => {
			super::push(out, 28021);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьви.
pub(crate) fn n1850(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1844);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 9238);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 28022);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 55490);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эви.
pub(crate) fn n1851(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1845);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 9239);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -вробумаги.
pub(crate) fn n122639(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121365);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 130721);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -абытнанги.
pub(crate) fn n122640(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121366);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 130722);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оги.
pub(crate) fn n1858(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1852);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 9257);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 9258);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 28045);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 55504);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 83632);
									s.is_empty()
								}
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
			super::push(out, 9259);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 28046);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 55505);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 83633);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 106088);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9260);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 28047);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 55506);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 9261);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 28048);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 28049);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55507);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 28050);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 55508);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 83634);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 106089);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 121367);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 9262);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28051);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 55509);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рги.
pub(crate) fn n1859(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1853);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 9263);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 28052);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 28053);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 55510);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 28054);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 28055);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55511);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 55512);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 83635);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 106090);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 121368);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уги.
pub(crate) fn n1860(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1854);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 9264);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 28056);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 55513);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 83636);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 106091);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 121369);
													match s {
														[] => true,
														['в', s @ ..] => super::t33::n132080(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 9265);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 28057);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 55514);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 83637);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 106092);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ньги.
pub(crate) fn n9293(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9266);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 28058);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 55515);
					match s {
						[] => true,
						['и', 'з', 'а', 'в', 'к', '\u{1}', s @ ..] => {
							super::push(out, 83638);
							s.is_empty()
						}
						['о', 'к', 'р', 'а', 'н', '\u{1}', s @ ..] => {
							super::push(out, 83639);
							s.is_empty()
						}
						['р', 'е', 'п', 'у', 'с', '\u{1}', s @ ..] => {
							super::push(out, 83640);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -бенди.
pub(crate) fn n28274(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28080);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 55527);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 83645);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 106096);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 121370);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кенди.
pub(crate) fn n28277(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28083);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 55528);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 83646);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 106097);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 121371);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -бренди.
pub(crate) fn n56102(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 55529);
	s.is_empty()
}

/// Words ending -фенди.
pub(crate) fn n28279(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28085);
	match s {
		[] => true,
		['ф', s @ ..] => {
			super::push(out, 55530);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 83647);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 55531);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -инди.
pub(crate) fn n9315(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9288);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 28086);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 28087);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 28088);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -унди.
pub(crate) fn n9316(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9289);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 28089);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 55532);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 83648);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 106098);
									s.is_empty()
								}
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
			super::push(out, 28090);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 55533);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 83649);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 106099);
									match s {
										[] => true,
										['я', s @ ..] => {
											super::push(out, 121372);
											s.is_empty()
										}
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
					super::push(out, 55534);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 83650);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 55535);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 83651);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -янди.
pub(crate) fn n9317(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9290);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 28091);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 55536);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 83652);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 106100);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оди.
pub(crate) fn n1867(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1861);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 9291);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 9292);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 28092);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 55537);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 9293);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 28093);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55538);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 83653);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рди.
pub(crate) fn n1868(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1862);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 9294);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 28094);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уди.
pub(crate) fn n1869(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1863);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9295);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 28095);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 9296);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 9297);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 28096);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 55539);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 83654);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 106101);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9298);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 28097);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 9299);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 28098);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 9300);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 9301);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 28099);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55540);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 9302);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 9303);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 9304);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -эди.
pub(crate) fn n1870(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1864);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 9305);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юди.
pub(crate) fn n1871(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1865);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 9306);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 28100);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 9307);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пассатижи.
pub(crate) fn n122647(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121373);
	s.is_empty()
}

/// Words ending -эстерхази.
pub(crate) fn n122648(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121374);
	s.is_empty()
}

/// Words ending -айзи.
pub(crate) fn n9365(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9338);
	match s {
		[] => true,
		['ч', s @ ..] => {
			super::push(out, 28137);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 55567);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 83673);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 106109);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 121375);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ейзи.
pub(crate) fn n9366(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9339);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 28138);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 55568);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мзи.
pub(crate) fn n1887(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1881);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9340);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 28139);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нзи.
pub(crate) fn n1888(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1882);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 9341);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 28140);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 55569);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ози.
pub(crate) fn n1889(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1883);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 9342);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 28141);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55570);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 83674);
									s.is_empty()
								}
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
			super::push(out, 9343);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 28142);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 28143);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -узи.
pub(crate) fn n1890(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1884);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 9344);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28144);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 55571);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 83675);
									s.is_empty()
								}
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
			super::push(out, 9345);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 28145);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цзи.
pub(crate) fn n1891(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1885);
	match s {
		[] => true,
		['й', s @ ..] => {
			super::push(out, 9346);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28146);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 55572);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юзи.
pub(crate) fn n1892(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1886);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 9347);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28147);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 55573);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 83676);
									s.is_empty()
								}
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
			super::push(out, 9348);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 28148);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -язи.
pub(crate) fn n1893(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1887);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 9349);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 28149);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55574);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 9350);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 28150);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55575);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оптеригии.
pub(crate) fn n122650(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121376);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 130724);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ехнологии.
pub(crate) fn n122651(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121377);
	s.is_empty()
}

/// Words ending -ол-лоджии.
pub(crate) fn n122652(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121378);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 130725);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ол-италии.
pub(crate) fn n122653(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121379);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 130726);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пол-армии.
pub(crate) fn n122654(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121380);
	s.is_empty()
}

/// Words ending -рмании.
pub(crate) fn n56157(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 55584);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 83686);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 106117);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 121381);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 130727);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -инии.
pub(crate) fn n9387(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9360);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 28161);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 55585);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 83687);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 106118);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 121382);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['а', s @ ..] => {
					super::push(out, 55586);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 83688);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 106119);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 121383);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кунии.
pub(crate) fn n28356(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28162);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 55587);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 83689);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рии.
pub(crate) fn n1901(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1895);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9362);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 28163);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 55588);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 83690);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 106120);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 121384);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 9363);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 28164);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55589);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сии.
pub(crate) fn n1902(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1896);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 9364);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 28165);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 55590);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 83691);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 106121);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 121385);
													match s {
														[] => true,
														['п', s @ ..] => super::t33::n132085(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хии.
pub(crate) fn n1903(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1897);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 9365);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28166);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 55591);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 83692);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 106122);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 121386);
													match s {
														[] => true,
														['о', s @ ..] => super::t33::n132086(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ции.
pub(crate) fn n1904(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1898);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 9366);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 28167);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 55592);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 83693);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 106123);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9367);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28168);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 55593);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 83694);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 106124);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 121387);
													match s {
														[] => true,
														['о', s @ ..] => super::t33::n132087(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -бки.
pub(crate) fn n1907(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1901);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9383);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 28185);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 9384);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 28186);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 55610);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 83706);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 55611);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 83707);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 106133);
											s.is_empty()
										}
										['ы', s @ ..] => {
											super::push(out, 106134);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 121388);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t33::n132088(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 9385);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 28187);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 55612);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 83708);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 106135);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 9386);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 28188);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 55613);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 83709);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 106136);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вки.
pub(crate) fn n1908(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1902);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9387);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 28189);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 55614);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 83710);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 106137);
											s.is_empty()
										}
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
					super::push(out, 28190);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 55615);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 9388);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 28191);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55616);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 55617);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 83711);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 55618);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 83712);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 28192);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55619);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 83713);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 28193);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55620);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 9389);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28194);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 55621);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 28195);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 9390);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28196);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55622);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 83714);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 28197);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 55623);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 83715);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дки.
pub(crate) fn n1909(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1903);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9391);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 28198);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55624);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 28199);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55625);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 83716);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 28200);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55626);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 83717);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 106138);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 121389);
													match s {
														[] => true,
														['е', s @ ..] => super::t33::n132089(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 9392);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28201);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 55627);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 83718);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 106139);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 121390);
													match s {
														[] => true,
														['п', s @ ..] => super::t33::n132090(s, out),
														_ => false,
													}
												}
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
							super::push(out, 55628);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 83719);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 106140);
											s.is_empty()
										}
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
					super::push(out, 28202);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 55629);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 83720);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 106141);
											s.is_empty()
										}
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
					super::push(out, 28203);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 55630);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 83721);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 106142);
											s.is_empty()
										}
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
					super::push(out, 28204);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55631);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 83722);
									s.is_empty()
								}
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
			super::push(out, 9393);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 28205);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55632);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 28206);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55633);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 9394);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28207);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55634);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 83723);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 28208);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55635);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 83724);
									s.is_empty()
								}
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
			super::push(out, 9395);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28209);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 55636);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 28210);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 55637);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 83725);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 106143);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жки.
pub(crate) fn n1911(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1905);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 9397);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 28212);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 55639);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 83727);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 106144);
											s.is_empty()
										}
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
					super::push(out, 28213);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 55640);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 83728);
									s.is_empty()
								}
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
			super::push(out, 9398);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28214);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 55641);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 83729);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 106145);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 121391);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 28215);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55642);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 55643);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', 'н', 'е', 'д', '\u{1}', s @ ..] => {
			super::push(out, 9399);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 9400);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 28216);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 55644);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 83730);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 106146);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зки.
pub(crate) fn n1912(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1906);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9401);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28217);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55645);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 83731);
									s.is_empty()
								}
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
			super::push(out, 9402);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 28218);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 55646);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 83732);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 106147);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 121392);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 28219);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55647);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 83733);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ики.
pub(crate) fn n1913(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1907);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 9403);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 28220);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 55648);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 83734);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 106148);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 121393);
													s.is_empty()
												}
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
							super::push(out, 55649);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 83735);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 106149);
											s.is_empty()
										}
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
					super::push(out, 28221);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 55650);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 83736);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 106150);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 121394);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 9404);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 28222);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 9405);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 28223);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 55651);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 83737);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 106151);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 121395);
													match s {
														[] => true,
														['р', s @ ..] => super::t33::n132091(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 28224);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 55652);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 83738);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 106152);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 121396);
													match s {
														[] => true,
														['и', s @ ..] => super::t33::n132092(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 9406);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 28225);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 55653);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 83739);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 106153);
											match s {
												[] => true,
												['э', s @ ..] => {
													super::push(out, 121397);
													match s {
														[] => true,
														['н', s @ ..] => super::t33::n132093(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 28226);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 55654);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 83740);
									s.is_empty()
								}
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
			super::push(out, 9407);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 28227);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 28228);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 55655);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 83741);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106154);
											s.is_empty()
										}
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
					super::push(out, 28229);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 55656);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 83742);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 106155);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 121398);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t33::n132094(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 28230);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 28231);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 55657);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 83743);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 106156);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 121399);
													s.is_empty()
												}
												['с', s @ ..] => {
													super::push(out, 121400);
													match s {
														[] => true,
														['е', s @ ..] => super::t33::n132095(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 28232);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => super::t33::n9435(s, out),
		['р', s @ ..] => super::t33::n9436(s, out),
		['с', s @ ..] => super::t33::n9437(s, out),
		['т', s @ ..] => super::t33::n9438(s, out),
		['ч', s @ ..] => super::t33::n9439(s, out),
		_ => false,
	}
}

/// Words ending -йки.
pub(crate) fn n1914(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1908);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9413);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 28241);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55663);
							s.is_empty()
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 28242);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55664);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 83749);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 106161);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9414);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 28243);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55665);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 9415);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 28244);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55666);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 9416);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28245);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55667);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 83750);
									s.is_empty()
								}
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
			super::push(out, 9417);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кки.
pub(crate) fn n1915(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1909);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 9418);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 28246);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 28247);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 9419);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 28248);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 9420);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 28249);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 55668);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 83751);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 106162);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лки.
pub(crate) fn n1916(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1910);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9421);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 28250);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 55669);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 83752);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 106163);
											s.is_empty()
										}
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
					super::push(out, 28251);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 28252);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 55670);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 83753);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 106164);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9422);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 28253);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 28254);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55671);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 28255);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 55672);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 83754);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106165);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9423);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 28256);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55673);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 28257);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55674);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 9424);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 28258);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55675);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 9425);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 28259);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55676);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 83755);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -емки.
pub(crate) fn n9453(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9426);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 28260);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нки.
pub(crate) fn n1918(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1912);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9427);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 28261);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55677);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 83756);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106166);
											s.is_empty()
										}
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
					super::push(out, 28262);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55678);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 28263);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 55679);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 28264);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 55680);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 28265);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 55681);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 83757);
									s.is_empty()
								}
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
			super::push(out, 9428);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 28266);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55682);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 83758);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 106167);
											s.is_empty()
										}
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
					super::push(out, 28267);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55683);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 83759);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 55684);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 83760);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 28268);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 28269);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55685);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 83761);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106168);
											s.is_empty()
										}
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
					super::push(out, 28270);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 9429);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 28271);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55686);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 28272);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55687);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 28273);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 55688);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 83762);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 106169);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 121405);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 28274);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 55689);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 83763);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 106170);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 121406);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 9430);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 28275);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55690);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 28276);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55691);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 83764);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106171);
											s.is_empty()
										}
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
					super::push(out, 28277);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 28278);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 55692);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 83765);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 106172);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 121407);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 9431);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 28279);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 55693);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 83766);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 55694);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 83767);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 106173);
											s.is_empty()
										}
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
					super::push(out, 28280);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 55695);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 83768);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 106174);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9432);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 28281);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 28282);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 55696);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 83769);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106175);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оки.
pub(crate) fn n1919(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1913);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 9433);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 28283);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 9434);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 28284);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 55697);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 83770);
									s.is_empty()
								}
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
			super::push(out, 9435);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 9436);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 28285);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 9437);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 28286);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 55698);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 28287);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55699);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 9438);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 28288);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55700);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 83771);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 106176);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 121408);
													match s {
														[] => true,
														['ь', s @ ..] => super::t33::n132099(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пки.
pub(crate) fn n1920(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1914);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 9439);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 28289);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 55701);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 83772);
									s.is_empty()
								}
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
			super::push(out, 9440);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28290);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 9441);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 28291);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 55702);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 83773);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 28292);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 55703);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 55704);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 83774);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106177);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9442);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 28293);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 9443);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 28294);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рки.
pub(crate) fn n1921(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1915);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 9444);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 28295);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55705);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 28296);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 55706);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 83775);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 28297);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55707);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 83776);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 28298);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 55708);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 83777);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 28299);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55709);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 83778);
									s.is_empty()
								}
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
			super::push(out, 9445);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 28300);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55710);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 83779);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 28301);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 55711);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 83780);
									s.is_empty()
								}
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
			super::push(out, 9446);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 28302);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 28303);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 55712);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 83781);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 28304);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 28305);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 9447);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 28306);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 55713);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 28307);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55714);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 83782);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 106178);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 121409);
													match s {
														[] => true,
														['м', s @ ..] => super::t33::n132100(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 28308);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55715);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 83783);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 9448);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 28309);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ски.
pub(crate) fn n1922(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1916);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 9449);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 28310);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 55716);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 83784);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 106179);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['а', s @ ..] => {
			super::push(out, 9450);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 28311);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55717);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 83785);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 28312);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 28313);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 55718);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 83786);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 106180);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 121410);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 9451);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 28314);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55719);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 28315);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55720);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 28316);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 55721);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 83787);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 28317);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 55722);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 83788);
									s.is_empty()
								}
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
			super::push(out, 9452);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 28318);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55723);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 28319);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 55724);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 83789);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 106181);
											s.is_empty()
										}
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
					super::push(out, 28320);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55725);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 55726);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 83790);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 28321);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 55727);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 83791);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 28322);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 55728);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 83792);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 106182);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 121411);
													match s {
														[] => true,
														['к', s @ ..] => super::t33::n132101(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 28323);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 28324);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 55729);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 83793);
									s.is_empty()
								}
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
			super::push(out, 9453);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28325);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55730);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 83794);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 106183);
											s.is_empty()
										}
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
					super::push(out, 28326);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 55731);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 83795);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 106184);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9454);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28327);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55732);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тки.
pub(crate) fn n1923(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1917);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9455);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 28328);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55733);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 83796);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106185);
											s.is_empty()
										}
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
					super::push(out, 28329);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 55734);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 28330);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55735);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 9456);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 28331);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55736);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 28332);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 9457);
			match s {
				[] => true,
				['ж', 'о', 'п', '\u{1}', s @ ..] => {
					super::push(out, 28333);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 28334);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 9458);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 28335);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55737);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 83797);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 106186);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 121412);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 28336);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 55738);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 83798);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 106187);
											s.is_empty()
										}
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
					super::push(out, 28337);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55739);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 83799);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 106188);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 121413);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 28338);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 55740);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 9459);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 28339);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 55741);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 83800);
									s.is_empty()
								}
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
			super::push(out, 9460);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 28340);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 55742);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 83801);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 83802);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 106189);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 121414);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 9461);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 28341);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 9462);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 28342);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 9463);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 28343);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 55743);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 83803);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 28344);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 55744);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 28345);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 55745);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 83804);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 106190);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 121415);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 28346);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 28347);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 55746);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уки.
pub(crate) fn n1924(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1918);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 9464);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28348);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 55747);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 28349);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55748);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 28350);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 55749);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 83805);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 106191);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 121416);
													match s {
														[] => true,
														['к', s @ ..] => super::t33::n132102(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 9465);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 9466);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 28351);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 55750);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 83806);
									s.is_empty()
								}
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
			super::push(out, 9467);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28352);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 55751);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 83807);
									s.is_empty()
								}
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
			super::push(out, 9468);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28353);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 55752);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 9469);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 28354);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 55753);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 83808);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 106192);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 121417);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 9470);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 28355);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 55754);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 83809);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чки.
pub(crate) fn n1925(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1919);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9471);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 28356);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 28357);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55755);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 83810);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 28358);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 55756);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 83811);
									s.is_empty()
								}
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
			super::push(out, 9472);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 28359);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 55757);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 83812);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 28360);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 55758);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 83813);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 106193);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 121418);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 28361);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 55759);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 83814);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 106194);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 121419);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 28362);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55760);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 28363);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 55761);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 83815);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 106195);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 121420);
													match s {
														[] => true,
														['ш', s @ ..] => super::t33::n132103(s, out),
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
									super::push(out, 83816);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 106196);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 121421);
													s.is_empty()
												}
												_ => false,
											}
										}
										['я', s @ ..] => {
											super::push(out, 106197);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 121422);
													s.is_empty()
												}
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
							super::push(out, 55762);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 83817);
									match s {
										[] => true,
										['я', s @ ..] => {
											super::push(out, 106198);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 121423);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 9473);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28364);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 55763);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 83818);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 28365);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 55764);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 83819);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 106199);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9474);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 28366);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 28367);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 28368);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55765);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 28369);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 55766);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 28370);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55767);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 83820);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 55768);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 83821);
									match s {
										[] => true,
										['й', s @ ..] => {
											super::push(out, 106200);
											match s {
												[] => true,
												['я', s @ ..] => {
													super::push(out, 121424);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 28371);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 55769);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 83822);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 28372);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 55770);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => super::t33::n28567(s, out),
				['т', s @ ..] => super::t33::n28568(s, out),
				['ч', s @ ..] => super::t33::n28569(s, out),
				_ => false,
			}
		}
		['ю', s @ ..] => super::t33::n9502(s, out),
		_ => false,
	}
}

/// Words ending -шки.
pub(crate) fn n1926(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1920);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9476);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 28378);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 28379);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 9477);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 28380);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55785);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 83833);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 106206);
											s.is_empty()
										}
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
					super::push(out, 28381);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55786);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 83834);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 55787);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 83835);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 28382);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 55788);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 83836);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 106207);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 121428);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 9478);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 28383);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55789);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 83837);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 55790);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 83838);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 28384);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55791);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 83839);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 106208);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 121429);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ю', s @ ..] => {
							super::push(out, 55792);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 83840);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 28385);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 28386);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55793);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 28387);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 55794);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 28388);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55795);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 28389);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 28390);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 55796);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 83841);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106209);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 55797);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 55798);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 83842);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 106210);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['я', s @ ..] => {
							super::push(out, 55799);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 83843);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 106211);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 121430);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 28391);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 55800);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 83844);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 106212);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ю', s @ ..] => {
							super::push(out, 55801);
							s.is_empty()
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 28392);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 9479);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28393);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 28394);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55802);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 83845);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 106213);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 55803);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 83846);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 106214);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 121431);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 9480);
			match s {
				[] => true,
				['г', s @ ..] => super::t33::n28589(s, out),
				['д', s @ ..] => {
					super::push(out, 28396);
					match s {
						[] => true,
						['а', s @ ..] => super::t33::n56379(s, out),
						_ => false,
					}
				}
				['м', s @ ..] => super::t33::n28591(s, out),
				['с', s @ ..] => super::t34::n28592(s, out),
				['т', s @ ..] => super::t34::n28593(s, out),
				_ => false,
			}
		}
		['э', s @ ..] => super::t34::n9508(s, out),
		['ю', s @ ..] => super::t34::n9509(s, out),
		['я', s @ ..] => super::t34::n9510(s, out),
		_ => false,
	}
}

/// Words ending -ьки.
pub(crate) fn n1927(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1921);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 9484);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28405);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 55815);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 83860);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 106223);
											s.is_empty()
										}
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
					super::push(out, 28406);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 55816);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 83861);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 106224);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 121436);
													s.is_empty()
												}
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
							super::push(out, 55817);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 83862);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 28407);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 55818);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 83863);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 28408);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 55819);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 83864);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 106225);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9485);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28409);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 55820);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 28410);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 55821);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 83865);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 106226);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 121437);
													match s {
														[] => true,
														['п', s @ ..] => super::t34::n132106(s, out),
														_ => false,
													}
												}
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
							super::push(out, 55822);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 83866);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 55823);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 83867);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 106227);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 121438);
													match s {
														[] => true,
														['е', s @ ..] => super::t34::n132107(s, out),
														_ => false,
													}
												}
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
							super::push(out, 55824);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 83868);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 28411);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 55825);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 55826);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 83869);
									s.is_empty()
								}
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
			super::push(out, 9486);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 28412);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 55827);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 83870);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 106228);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юки.
pub(crate) fn n1928(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1922);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 9487);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 28413);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 55828);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 83871);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 106229);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 121439);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 9488);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 28414);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яки.
pub(crate) fn n1929(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1923);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9489);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 28415);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55829);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 83872);
									s.is_empty()
								}
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
			super::push(out, 9490);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 28416);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 55830);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 9491);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 28417);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 55831);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 83873);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 28418);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55832);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 83874);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 106230);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 121440);
													match s {
														[] => true,
														['о', s @ ..] => super::t34::n132108(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 28419);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 55833);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 83875);
									s.is_empty()
								}
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
			super::push(out, 9492);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 28420);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 55834);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 83876);
									s.is_empty()
								}
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
			super::push(out, 9493);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 28421);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55835);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 83877);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цинандали.
pub(crate) fn n122715(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121441);
	s.is_empty()
}

/// Words ending -апсандали.
pub(crate) fn n122716(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121442);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 130752);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 136198);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рали.
pub(crate) fn n9531(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9504);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 28442);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 55851);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 28443);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тали.
pub(crate) fn n9532(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9505);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 28444);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 28445);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 55852);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 83888);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 55853);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 28446);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 55854);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 83889);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хали.
pub(crate) fn n9533(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9506);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 28447);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 55855);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шали.
pub(crate) fn n9534(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9507);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 28448);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 55856);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 83890);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -бли.
pub(crate) fn n1931(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1925);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9508);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 28449);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 55857);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 83891);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 55858);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 83892);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 28450);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55859);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 9509);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 28451);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 55860);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 9510);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 28452);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55861);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 83893);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 106234);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 121443);
													match s {
														[] => true,
														['н', s @ ..] => super::t34::n132110(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гли.
pub(crate) fn n1932(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1926);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 9511);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 28453);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 9512);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 28454);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 55862);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 83894);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 28455);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 55863);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дли.
pub(crate) fn n1933(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1927);
	match s {
		[] => true,
		['э', s @ ..] => {
			super::push(out, 9513);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 28456);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 55864);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ели.
pub(crate) fn n1934(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1928);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 9514);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 28457);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 28458);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 55865);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 83895);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 106235);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9515);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 28459);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 55866);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 83896);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 106236);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9516);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 28460);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 55867);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 83897);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 106237);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9517);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28461);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55868);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 9518);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 28462);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 55869);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 83898);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 106238);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ц', s @ ..] => {
							super::push(out, 55870);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 83899);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 106239);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 121444);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 28463);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 55871);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 83900);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106240);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9519);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 9520);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28464);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 55872);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зли.
pub(crate) fn n1935(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1929);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 9521);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 28465);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 55873);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -или.
pub(crate) fn n1936(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1930);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9522);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 9523);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 28466);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55874);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 83901);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 106241);
											match s {
												[] => true,
												['ч', s @ ..] => {
													super::push(out, 121445);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 9524);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 28467);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 55875);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 83902);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 106242);
											s.is_empty()
										}
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
					super::push(out, 28468);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55876);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 83903);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 106243);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 121446);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 9525);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 9526);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 28469);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 9527);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28470);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 55877);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 83904);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 28471);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 55878);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 83905);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 106244);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9528);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 28472);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55879);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 83906);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лли.
pub(crate) fn n1937(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1931);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9529);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 28473);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55880);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 83907);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 55881);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 83908);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 28474);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 9530);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 28475);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 28476);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 28477);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 55882);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 83909);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 106245);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 121447);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 28478);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 55883);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 83910);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 106246);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 121448);
													match s {
														[] => true,
														['т', s @ ..] => super::t34::n132111(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 9531);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 28479);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55884);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 83911);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 106247);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 121449);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t34::n132112(s, out),
														_ => false,
													}
												}
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
							super::push(out, 55885);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 83912);
									match s {
										[] => true,
										['й', s @ ..] => {
											super::push(out, 106248);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 121450);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['х', s @ ..] => {
									super::push(out, 83913);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 106249);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 121451);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 28480);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 28481);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 28482);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55886);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 83914);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 28483);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 9532);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 28484);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 28485);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 9533);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 28486);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мли.
pub(crate) fn n1938(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1932);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 9534);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 28487);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 55887);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 83915);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 106250);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 121452);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оли.
pub(crate) fn n1939(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1933);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9535);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 9536);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 28488);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55888);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 83916);
									s.is_empty()
								}
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
			super::push(out, 9537);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28489);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 9538);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 28490);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55889);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 83917);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 106251);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9539);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 9540);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28491);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 55890);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 9541);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 28492);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 28493);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 55891);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 83918);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 106252);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 121453);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 9542);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 28494);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 55892);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 9543);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 28495);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 55893);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 83919);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 106253);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 121454);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пли.
pub(crate) fn n1940(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1934);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9544);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 28496);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 28497);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 9545);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 28498);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 28499);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55894);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рли.
pub(crate) fn n1941(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1935);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 9546);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 28500);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 9547);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 28501);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55895);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 55896);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сли.
pub(crate) fn n1942(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1936);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9548);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 28502);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55897);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 9549);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28503);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 9550);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 28504);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 55898);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 9551);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 28505);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 9552);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 28506);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 9553);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 28507);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 28508);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 9554);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 28509);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 9555);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тли.
pub(crate) fn n1943(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1937);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 9556);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 28510);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 55899);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 9557);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28511);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 55900);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ули.
pub(crate) fn n1944(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1938);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9558);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 28512);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55901);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 83920);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 106254);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 121455);
													match s {
														[] => true,
														['н', s @ ..] => super::t34::n132113(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 28513);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 55902);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 9559);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 28514);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55903);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 83921);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 106255);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 121456);
													match s {
														[] => true,
														['р', s @ ..] => super::t34::n132114(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 9560);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 28515);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 55904);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 55905);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 9561);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 28516);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 55906);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 83922);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 106256);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 121457);
													match s {
														[] => true,
														['к', s @ ..] => super::t34::n132115(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 9562);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 28517);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 9563);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 28518);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 55907);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 55908);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 83923);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 106257);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9564);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 28519);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 55909);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 9565);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 9566);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28520);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 55910);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фли.
pub(crate) fn n1945(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1939);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 9567);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 28521);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 55911);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 83924);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 106258);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 121458);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шли.
pub(crate) fn n1946(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1940);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9568);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 28522);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 9569);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыли.
pub(crate) fn n1947(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1941);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 9570);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 28523);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 55912);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 83925);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юли.
pub(crate) fn n1949(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1943);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 9571);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 28524);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 55913);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 83926);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 106259);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9572);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 9573);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 28525);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 9574);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 28526);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 55914);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 83927);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яли.
pub(crate) fn n1950(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1944);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 9575);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 28527);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 55915);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 83928);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мвойсками.
pub(crate) fn n122733(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121459);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 130759);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 136201);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -равделами.
pub(crate) fn n122734(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121460);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 130760);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьмирснами.
pub(crate) fn n122735(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121461);
	s.is_empty()
}

/// Words ending -а-байрами.
pub(crate) fn n122736(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121462);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 130761);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -еми.
pub(crate) fn n1953(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1947);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 9587);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 28544);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 55929);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 83939);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 106265);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ими.
pub(crate) fn n1954(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1948);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 9588);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 28545);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 55930);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 83940);
									s.is_empty()
								}
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
			super::push(out, 9589);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 28546);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 55931);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 83941);
									s.is_empty()
								}
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
			super::push(out, 9590);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 28547);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 28548);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 55932);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 83942);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 106266);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 9591);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28549);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 55933);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 9592);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 28550);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 55934);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

