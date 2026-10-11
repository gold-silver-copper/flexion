use flexion::Path;

/// Words ending -мь.
pub(crate) fn n537(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 536);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 4643);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 18947);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 18948);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 43191);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 18949);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 43192);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 71452);
									s.is_empty()
								}
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
			super::push(out, 4644);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 18950);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 43193);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 4645);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 18951);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 18952);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 18953);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 43194);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 71453);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 96505);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 114917);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 18954);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 43195);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 4646);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 18955);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 43196);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 71454);
									s.is_empty()
								}
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
			super::push(out, 4647);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18956);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 43197);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 4648);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 18957);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 43198);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 71455);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нь.
pub(crate) fn n538(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 537);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4649);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 18958);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 43199);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 43200);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 43201);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 71456);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ю', s @ ..] => {
							super::push(out, 43202);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 71457);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 96506);
											s.is_empty()
										}
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
					super::push(out, 18959);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 43203);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 71458);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 43204);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 71459);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 96507);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ч', s @ ..] => {
									super::push(out, 71460);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 96508);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 114918);
													match s {
														[] => true,
														['а', s @ ..] => super::t27::n127994(s, out),
														_ => false,
													}
												}
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
							super::push(out, 43205);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 71461);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 96509);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['й', s @ ..] => {
							super::push(out, 43206);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 71462);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 96510);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 43207);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 71463);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 96511);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ь', s @ ..] => super::t27::n43631(s, out),
						_ => false,
					}
				}
				['г', s @ ..] => super::t27::n19065(s, out),
				['д', s @ ..] => super::t27::n19066(s, out),
				['з', s @ ..] => super::t27::n19067(s, out),
				['к', s @ ..] => super::t27::n19068(s, out),
				['л', s @ ..] => super::t27::n19069(s, out),
				['м', s @ ..] => super::t27::n19070(s, out),
				['н', s @ ..] => super::t27::n19071(s, out),
				['о', s @ ..] => super::t27::n19072(s, out),
				['п', s @ ..] => super::t27::n19073(s, out),
				['р', s @ ..] => super::t27::n19074(s, out),
				['т', s @ ..] => super::t27::n19075(s, out),
				['у', s @ ..] => super::t27::n19076(s, out),
				['ф', s @ ..] => super::t27::n19077(s, out),
				['х', s @ ..] => super::t27::n19078(s, out),
				['ш', s @ ..] => super::t27::n19079(s, out),
				['ю', s @ ..] => super::t27::n19080(s, out),
				_ => false,
			}
		}
		['г', s @ ..] => super::t27::n4661(s, out),
		['е', s @ ..] => super::t27::n4662(s, out),
		['з', s @ ..] => super::t27::n4663(s, out),
		['и', s @ ..] => super::t27::n4664(s, out),
		['о', s @ ..] => super::t27::n4665(s, out),
		['р', s @ ..] => super::t27::n4666(s, out),
		['с', s @ ..] => super::t27::n4667(s, out),
		['у', s @ ..] => super::t27::n4668(s, out),
		['ы', s @ ..] => super::t27::n4669(s, out),
		['э', s @ ..] => super::t27::n4670(s, out),
		['ю', s @ ..] => super::t27::n4671(s, out),
		['я', s @ ..] => super::t27::n4672(s, out),
		_ => false,
	}
}

/// Words ending -пь.
pub(crate) fn n539(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 538);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 4662);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 19038);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 43405);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 19039);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 43406);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 71706);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 71707);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 96720);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 115038);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 19040);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 43407);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 43408);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 71708);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 43409);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 71709);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 43410);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 71710);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 43411);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 71711);
									s.is_empty()
								}
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
			super::push(out, 4663);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 19041);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 43412);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 71712);
									s.is_empty()
								}
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
			super::push(out, 4664);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 19042);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 43413);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 19043);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 19044);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 43414);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 71713);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 96721);
											s.is_empty()
										}
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
					super::push(out, 19045);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 4665);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 19046);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 43415);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 19047);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 71714);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 96722);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 115039);
													match s {
														[] => true,
														['п', s @ ..] => super::t27::n128052(s, out),
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
									super::push(out, 71715);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 96723);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 19048);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 43416);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 4666);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 19049);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 19050);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 43417);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 71716);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 43418);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 71717);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 96724);
											match s {
												[] => true,
												['п', '\u{1}', s @ ..] => {
													super::push(out, 115040);
													s.is_empty()
												}
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
							super::push(out, 43419);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 43420);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 43421);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рь.
pub(crate) fn n540(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 539);
	match s {
		[] => true,
		['а', s @ ..] => super::t27::n4678(s, out),
		['б', s @ ..] => {
			super::push(out, 4668);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19069);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 43510);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 71837);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 43511);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 71838);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 96807);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 115066);
													match s {
														[] => true,
														['е', 'н', '\u{1}', s @ ..] => super::t27::n128064(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 19070);
					s.is_empty()
				}
				['ю', s @ ..] => {
					super::push(out, 19071);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 43512);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 71839);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 96808);
											s.is_empty()
										}
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
					super::push(out, 19072);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 43513);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 4669);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 19073);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 19074);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 43514);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 71840);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 71841);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 96809);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 115067);
													match s {
														[] => true,
														['е', s @ ..] => super::t27::n128065(s, out),
														_ => false,
													}
												}
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
							super::push(out, 43515);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 71842);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 43516);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 71843);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 96810);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 115068);
													match s {
														[] => true,
														['т', s @ ..] => super::t27::n128066(s, out),
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
									super::push(out, 71844);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 96811);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 43517);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 71845);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 43518);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 71846);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 19075);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 43519);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 71847);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 96812);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 115069);
													match s {
														[] => true,
														['д', s @ ..] => super::t27::n128067(s, out),
														_ => false,
													}
												}
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
							super::push(out, 43520);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 71848);
									s.is_empty()
								}
								['ъ', s @ ..] => {
									super::push(out, 71849);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 96813);
											match s {
												[] => true,
												['ь', s @ ..] => {
													super::push(out, 115070);
													match s {
														[] => true,
														['л', s @ ..] => super::t27::n128068(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => super::t27::n19181(s, out),
				['т', s @ ..] => super::t27::n19182(s, out),
				['х', s @ ..] => super::t27::n19183(s, out),
				['ч', s @ ..] => super::t27::n19184(s, out),
				['щ', s @ ..] => super::t27::n19185(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t27::n4681(s, out),
		['о', s @ ..] => super::t27::n4682(s, out),
		['п', s @ ..] => super::t27::n4683(s, out),
		['р', s @ ..] => super::t27::n4684(s, out),
		['у', s @ ..] => super::t27::n4685(s, out),
		['х', s @ ..] => super::t27::n4686(s, out),
		['ы', s @ ..] => super::t27::n4687(s, out),
		['я', s @ ..] => super::t27::n4688(s, out),
		_ => false,
	}
}

/// Words ending -сь.
pub(crate) fn n541(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 540);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4678);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 19114);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 43589);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 71912);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 19115);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 43590);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 71913);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 96856);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 115094);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 19116);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 43591);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 71914);
									s.is_empty()
								}
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
			super::push(out, 4679);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 19117);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 43592);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 71915);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 71916);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 96857);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 115095);
													s.is_empty()
												}
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
							super::push(out, 43593);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 71917);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 19118);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 43594);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 71918);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 19119);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 43595);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 71919);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 96858);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 43596);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 71920);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 96859);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 43597);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 71921);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 43598);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 71922);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 19120);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 43599);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 43600);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 71923);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 19121);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 43601);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 19122);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 43602);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 71924);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 43603);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 71925);
									s.is_empty()
								}
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
			super::push(out, 4680);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 19123);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 43604);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 19124);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 43605);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 71926);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 96860);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 115096);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => super::t27::n44030(s, out),
						_ => false,
					}
				}
				['п', s @ ..] => super::t27::n19230(s, out),
				['р', s @ ..] => super::t27::n19231(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t27::n4692(s, out),
		['у', s @ ..] => super::t27::n4693(s, out),
		['ы', s @ ..] => super::t27::n4694(s, out),
		['я', s @ ..] => super::t27::n4695(s, out),
		_ => false,
	}
}

/// Words ending -ть.
pub(crate) fn n542(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 541);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4685);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 19142);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 43633);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 71955);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 96886);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 115107);
													match s {
														[] => true,
														['н', s @ ..] => super::t27::n128080(s, out),
														['о', s @ ..] => super::t27::n128081(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 19143);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 19144);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 43634);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 71956);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 96887);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 115108);
													match s {
														[] => true,
														['б', s @ ..] => super::t27::n128082(s, out),
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
									super::push(out, 71957);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 19145);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 43635);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 43636);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 19146);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 43637);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 71958);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 43638);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 71959);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 19147);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 43639);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 43640);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 71960);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 96888);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 115109);
													match s {
														[] => true,
														['л', s @ ..] => super::t27::n128083(s, out),
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
									super::push(out, 71961);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 96889);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 115110);
													match s {
														[] => true,
														['г', s @ ..] => super::t27::n128084(s, out),
														_ => false,
													}
												}
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
							super::push(out, 43641);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 71962);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 96890);
											s.is_empty()
										}
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
					super::push(out, 19148);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 43642);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 19149);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 43643);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 71963);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 19150);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 43644);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 19151);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 43645);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 43646);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 19152);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 43647);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 71964);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t27::n4697(s, out),
		['и', s @ ..] => super::t27::n4698(s, out),
		['л', s @ ..] => super::t27::n4699(s, out),
		['о', s @ ..] => super::t27::n4700(s, out),
		['р', s @ ..] => super::t27::n4701(s, out),
		['с', s @ ..] => super::t27::n4702(s, out),
		['у', s @ ..] => super::t27::n4703(s, out),
		['ф', s @ ..] => super::t27::n4704(s, out),
		['ы', s @ ..] => super::t27::n4705(s, out),
		['ю', s @ ..] => {
			super::push(out, 4695);
			match s {
				[] => true,
				['л', s @ ..] => super::t27::n19309(s, out),
				_ => false,
			}
		}
		['я', s @ ..] => super::t27::n4707(s, out),
		_ => false,
	}
}

/// Words ending -фь.
pub(crate) fn n543(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 542);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 4697);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 19208);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 43759);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 4698);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 19209);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 43760);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чь.
pub(crate) fn n544(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 543);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 4699);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 19210);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 43761);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 43762);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 72164);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 97204);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ч', s @ ..] => {
									super::push(out, 72165);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 97205);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 115632);
													s.is_empty()
												}
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
							super::push(out, 43763);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 72166);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 97206);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 115633);
													match s {
														[] => true,
														['х', s @ ..] => super::t27::n128805(s, out),
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
									super::push(out, 72167);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 97207);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 115634);
													match s {
														[] => true,
														['е', s @ ..] => super::t27::n128806(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 19211);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 43764);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 72168);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 97208);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 115635);
													s.is_empty()
												}
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
							super::push(out, 43765);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 72169);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 97209);
											s.is_empty()
										}
										_ => false,
									}
								}
								['м', s @ ..] => {
									super::push(out, 72170);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 97210);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 115636);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 19212);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 19213);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 43766);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 72171);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 97211);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4700);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 19214);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 4701);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 19215);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 43767);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 72172);
									s.is_empty()
								}
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
			super::push(out, 4702);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 19216);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 43768);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 19217);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 43769);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 19218);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 43770);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 72173);
									s.is_empty()
								}
								['щ', s @ ..] => {
									super::push(out, 72174);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 43771);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 72175);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 97212);
											match s {
												[] => true,
												['з', s @ ..] => super::t27::n116884(s, out),
												['н', s @ ..] => {
													super::push(out, 115638);
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => super::t27::n98253(s, out),
										_ => false,
									}
								}
								['т', s @ ..] => super::t27::n72930(s, out),
								_ => false,
							}
						}
						['с', s @ ..] => super::t27::n44196(s, out),
						_ => false,
					}
				}
				['м', s @ ..] => super::t27::n19324(s, out),
				['н', s @ ..] => super::t27::n19325(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => super::t27::n4714(s, out),
		['у', s @ ..] => super::t27::n4715(s, out),
		_ => false,
	}
}

/// Words ending -шь.
pub(crate) fn n545(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 544);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4705);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 19223);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 43780);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 72183);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 97219);
											s.is_empty()
										}
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
					super::push(out, 19224);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 43781);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 4706);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 19225);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 43782);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 19226);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 43783);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 4707);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 19227);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 43784);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 43785);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 72184);
									s.is_empty()
								}
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
			super::push(out, 4708);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 19228);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 19229);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 43786);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 72185);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 19230);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 43787);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 72186);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 97220);
											s.is_empty()
										}
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
					super::push(out, 19231);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 43788);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 72187);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 19232);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 43789);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 19233);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 43790);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 72188);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 19234);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 43791);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 72189);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 43792);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 72190);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 97221);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4709);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19235);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 43793);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 4710);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 19236);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 43794);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 19237);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 43795);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 72191);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 97222);
											s.is_empty()
										}
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
					super::push(out, 19238);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 19239);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 43796);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 72192);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 19240);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 4711);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 19241);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 43797);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 72193);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 97223);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 115644);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 19242);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 43798);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 72194);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 97224);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 115645);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4712);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 19243);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 43799);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 72195);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щь.
pub(crate) fn n546(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 545);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 4713);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 19244);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 19245);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 43800);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 4714);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 19246);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 43801);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 4715);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 19247);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 43802);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 72196);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 43803);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 72197);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 97225);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 115646);
													match s {
														[] => true,
														['т', s @ ..] => super::t27::n128810(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['х', s @ ..] => {
											super::push(out, 97226);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 115647);
													match s {
														[] => true,
														['с', s @ ..] => super::t27::n128811(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 19248);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 43804);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 43805);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 72198);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дзонг-кэ.
pub(crate) fn n116895(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115648);
	s.is_empty()
}

/// Words ending -ва-тэ.
pub(crate) fn n44244(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43820);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 72204);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 97228);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 115649);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 127466);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -отэ.
pub(crate) fn n4753(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4742);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 19275);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уэ.
pub(crate) fn n561(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 560);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 4743);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 19276);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хэ.
pub(crate) fn n562(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 561);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 4744);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19277);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 43821);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 72205);
									s.is_empty()
								}
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
			super::push(out, 4745);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 19278);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 4746);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 19279);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 43822);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 72206);
									match s {
										[] => true,
										['й', s @ ..] => {
											super::push(out, 97229);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 115650);
													match s {
														[] => true,
														['с', s @ ..] => super::t27::n128813(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яэ.
pub(crate) fn n564(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 563);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 4747);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19280);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 43823);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 72207);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 97230);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 115651);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -айс-ревю.
pub(crate) fn n116899(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115652);
	s.is_empty()
}

/// Words ending -сю.
pub(crate) fn n577(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 576);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 4769);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 19304);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 43840);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 4770);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 19305);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 43841);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 4771);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 19306);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ютю.
pub(crate) fn n4783(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4772);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 19307);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -цю.
pub(crate) fn n579(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 578);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 4773);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 19308);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 43842);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 72216);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 97234);
											match s {
												[] => true,
												['й', s @ ..] => {
													super::push(out, 115653);
													match s {
														[] => true,
														['ю', s @ ..] => super::t27::n128814(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чю.
pub(crate) fn n580(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 579);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 4774);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 19309);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 43843);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 72217);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 97235);
											match s {
												[] => true,
												['й', s @ ..] => {
													super::push(out, 115654);
													match s {
														[] => true,
														['ю', s @ ..] => super::t27::n128815(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шю.
pub(crate) fn n581(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 580);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 4775);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 19310);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ью.
pub(crate) fn n582(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 581);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 4776);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 19311);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 43844);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 19312);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 43845);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 72218);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 19313);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 43846);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 72219);
									s.is_empty()
								}
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
			super::push(out, 4777);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 19314);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 43847);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 72220);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 19315);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 43848);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 72221);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 97236);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4778);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 19316);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 43849);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 72222);
									s.is_empty()
								}
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
			super::push(out, 4779);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 19317);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 43850);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 72223);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 97237);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 115655);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 19318);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 43851);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 72224);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 97238);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 115656);
													match s {
														[] => true,
														['р', s @ ..] => super::t27::n128816(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4780);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 19319);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 43852);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 4781);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 19320);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 43853);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 72225);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ая.
pub(crate) fn n583(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 582);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4782);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 4783);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19321);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 43854);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 72226);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 97239);
											s.is_empty()
										}
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
					super::push(out, 19322);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 43855);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 72227);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 97240);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 115657);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 72228);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 43856);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 72229);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 97241);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ь', s @ ..] => {
							super::push(out, 43857);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 19323);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 43858);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 72230);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 43859);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 72231);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 97242);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 115658);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => super::t27::n19429(s, out),
				['с', s @ ..] => {
					super::push(out, 19325);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 43871);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 19326);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 43872);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 72256);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 97268);
											s.is_empty()
										}
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
					super::push(out, 19327);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 43873);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 72257);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 97269);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 115675);
													match s {
														[] => true,
														['к', s @ ..] => super::t27::n128827(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4784);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 19328);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 43874);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 72258);
									s.is_empty()
								}
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
			super::push(out, 4785);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 19329);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 43875);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 72259);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 97270);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => super::t27::n4797(s, out),
		['з', s @ ..] => super::t28::n4798(s, out),
		['к', s @ ..] => super::t28::n4799(s, out),
		['л', s @ ..] => super::t28::n4800(s, out),
		['м', s @ ..] => super::t28::n4801(s, out),
		['н', s @ ..] => super::t28::n4802(s, out),
		['п', s @ ..] => super::t28::n4803(s, out),
		['р', s @ ..] => super::t28::n4804(s, out),
		['с', s @ ..] => super::t28::n4805(s, out),
		['т', s @ ..] => super::t28::n4806(s, out),
		['ф', s @ ..] => {
			super::push(out, 4796);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 19385);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => super::t28::n4808(s, out),
		['ч', s @ ..] => super::t28::n4809(s, out),
		['ш', s @ ..] => super::t28::n4810(s, out),
		['щ', s @ ..] => super::t28::n4811(s, out),
		_ => false,
	}
}

/// Words ending -ея.
pub(crate) fn n585(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 584);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 4806);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 4807);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19411);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 19412);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 44030);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 72485);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 97490);
											s.is_empty()
										}
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
					super::push(out, 19413);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 44031);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 4808);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 19414);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 19415);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 44032);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 72486);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 97491);
											s.is_empty()
										}
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
					super::push(out, 19416);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 44033);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 72487);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 97492);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 44034);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 72488);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 44035);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 72489);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 19417);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 19418);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 44036);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 19419);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 4809);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 19420);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 44037);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 72490);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 97493);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 115867);
													match s {
														[] => true,
														['п', s @ ..] => super::t28::n128952(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 19421);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 44038);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 72491);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 44039);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 72492);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 97494);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 115868);
													match s {
														[] => true,
														['п', s @ ..] => super::t28::n128953(s, out),
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
									super::push(out, 72493);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 97495);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 72494);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 97496);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 115869);
													match s {
														[] => true,
														['х', s @ ..] => super::t28::n128954(s, out),
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
									super::push(out, 72495);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 97497);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 72496);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 97498);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 72497);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 97499);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 72498);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 44040);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 72499);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 97500);
											s.is_empty()
										}
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
					super::push(out, 19422);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 44041);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 4810);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 19423);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 44042);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 72500);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 97501);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4811);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19424);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 44043);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 72501);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => super::t28::n4823(s, out),
		['м', s @ ..] => super::t28::n4824(s, out),
		['н', s @ ..] => super::t28::n4825(s, out),
		['п', s @ ..] => {
			super::push(out, 4815);
			match s {
				[] => true,
				['о', s @ ..] => super::t28::n19547(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => super::t28::n4827(s, out),
		['с', s @ ..] => super::t28::n4828(s, out),
		['т', s @ ..] => super::t28::n4829(s, out),
		['ф', s @ ..] => super::t28::n4830(s, out),
		['х', s @ ..] => super::t28::n4831(s, out),
		['ц', s @ ..] => super::t28::n4832(s, out),
		['ч', s @ ..] => super::t28::n4833(s, out),
		['ш', s @ ..] => {
			super::push(out, 4823);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ия.
pub(crate) fn n587(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 586);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4831);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 19482);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 44115);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 72552);
									s.is_empty()
								}
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
			super::push(out, 4832);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19483);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 44116);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 72553);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 44117);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 72554);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 97536);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 115885);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 19484);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 44118);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 44119);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 44120);
							s.is_empty()
						}
						['ф', s @ ..] => {
							super::push(out, 44121);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 72555);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 97537);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 115886);
													match s {
														[] => true,
														['н', s @ ..] => super::t28::n128965(s, out),
														['р', s @ ..] => super::t28::n128966(s, out),
														['т', s @ ..] => super::t28::n128967(s, out),
														['ь', s @ ..] => super::t28::n128968(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 19485);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 44122);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 72556);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 19486);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 44123);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 72557);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 44124);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 72558);
									s.is_empty()
								}
								['о', s @ ..] => super::t28::n73313(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 19487);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 44125);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 19488);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 44126);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 72560);
									match s {
										[] => true,
										['ж', s @ ..] => super::t28::n98587(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => super::t28::n4844(s, out),
		['г', s @ ..] => super::t28::n4845(s, out),
		['д', s @ ..] => super::t28::n4846(s, out),
		['ж', s @ ..] => super::t28::n4847(s, out),
		['з', s @ ..] => super::t28::n4848(s, out),
		['к', s @ ..] => super::t28::n4849(s, out),
		['л', s @ ..] => super::t28::n4850(s, out),
		['м', s @ ..] => super::t28::n4851(s, out),
		['н', s @ ..] => super::t28::n4852(s, out),
		['п', s @ ..] => super::t28::n4853(s, out),
		['р', s @ ..] => super::t28::n4854(s, out),
		['с', s @ ..] => super::t28::n4855(s, out),
		['т', s @ ..] => super::t28::n4856(s, out),
		['ф', s @ ..] => super::t28::n4857(s, out),
		['х', s @ ..] => super::t28::n4858(s, out),
		['ц', s @ ..] => super::t28::n4859(s, out),
		['ч', s @ ..] => super::t28::n4860(s, out),
		['ш', s @ ..] => super::t28::n4861(s, out),
		_ => false,
	}
}

/// Words ending -байя.
pub(crate) fn n19757(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19651);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 44620);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -вайя.
pub(crate) fn n19758(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19652);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 44621);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кайя.
pub(crate) fn n19759(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19653);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 44622);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 73298);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 98324);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лайя.
pub(crate) fn n19760(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19654);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 44623);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 73299);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 98325);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 116566);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 128080);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 134856);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 73300);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 44624);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 73301);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -майя.
pub(crate) fn n19761(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19655);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 44625);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пайя.
pub(crate) fn n19762(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19656);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 44626);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 73302);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 44627);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 73303);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 98326);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -райя.
pub(crate) fn n19763(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19657);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 44628);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 44629);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 73304);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сайя.
pub(crate) fn n19764(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19658);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 44630);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 73305);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тайя.
pub(crate) fn n19765(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19659);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 44631);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 73306);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 44632);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 73307);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 98327);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хайя.
pub(crate) fn n19766(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19660);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 44633);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 73308);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 98328);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 116567);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ойя.
pub(crate) fn n4863(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4852);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 19661);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 44634);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 73309);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 98329);
									s.is_empty()
								}
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
			super::push(out, 19662);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 44635);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 73310);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 19663);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 44636);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 73311);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 98330);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 44637);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 73312);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 98331);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 116568);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 128081);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 44638);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 73313);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 98332);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 116569);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 19664);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уйя.
pub(crate) fn n4864(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4853);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 19665);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 44639);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 19666);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 44640);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 73314);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 98333);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 116570);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 19667);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 44641);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 73315);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 98334);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ля.
pub(crate) fn n589(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 588);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 4854);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 19668);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 44642);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 73316);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 98335);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 116571);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4855);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 19669);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 19670);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 44643);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 19671);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 19672);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 19673);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 44644);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 19674);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 4856);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19675);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 44645);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 19676);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 44646);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 73317);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 19677);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 44647);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 73318);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											super::push(out, 98336);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4857);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19678);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 44648);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 19679);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 44649);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 73319);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 44650);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 73320);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 98337);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4858);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 19680);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 44651);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 19681);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 44652);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 73321);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 44653);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 73322);
									s.is_empty()
								}
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
			super::push(out, 4859);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 19682);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 19683);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 19684);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 44654);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 73323);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 44655);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 73324);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 44656);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 73325);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 19685);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 19686);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 19687);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 44657);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 44658);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 73326);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 44659);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 73327);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 98338);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 116572);
													match s {
														[] => true,
														['п', s @ ..] => super::t28::n129432(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 19688);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 44660);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 73328);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 98339);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 116573);
													match s {
														[] => true,
														['о', s @ ..] => super::t28::n129433(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 19689);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 44661);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 73329);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 98340);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 116574);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => super::t28::n19796(s, out),
				['ч', s @ ..] => {
					super::push(out, 19691);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t28::n4871(s, out),
		['к', s @ ..] => super::t28::n4872(s, out),
		['л', s @ ..] => super::t28::n4873(s, out),
		['м', s @ ..] => super::t28::n4874(s, out),
		['о', s @ ..] => super::t28::n4875(s, out),
		['п', s @ ..] => super::t28::n4876(s, out),
		['р', s @ ..] => {
			super::push(out, 4866);
			match s {
				[] => true,
				['а', s @ ..] => super::t28::n19824(s, out),
				_ => false,
			}
		}
		['с', s @ ..] => super::t28::n4878(s, out),
		['т', s @ ..] => super::t28::n4879(s, out),
		['у', s @ ..] => super::t28::n4880(s, out),
		['ф', s @ ..] => super::t28::n4881(s, out),
		['х', s @ ..] => super::t28::n4882(s, out),
		['ы', s @ ..] => super::t28::n4883(s, out),
		['э', s @ ..] => super::t28::n4884(s, out),
		['ю', s @ ..] => super::t28::n4885(s, out),
		['я', s @ ..] => super::t28::n4886(s, out),
		_ => false,
	}
}

/// Words ending -мя.
pub(crate) fn n590(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 589);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4876);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 19750);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 44742);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 19751);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 44743);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 19752);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 4877);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 19753);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 44744);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 19754);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 44745);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 44746);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 73398);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 98376);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 116596);
													match s {
														[] => true,
														['т', s @ ..] => super::t28::n129443(s, out),
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
									super::push(out, 73399);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 98377);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 116597);
													match s {
														[] => true,
														['а', s @ ..] => super::t28::n129444(s, out),
														_ => false,
													}
												}
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
							super::push(out, 44747);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 73400);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 44748);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 73401);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 19755);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 44749);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 73402);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 98378);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 116598);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 19756);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 4878);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 4879);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 19757);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 19758);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 44750);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 73403);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 98379);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4880);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 19759);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 44751);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 73404);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ня.
pub(crate) fn n591(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 590);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4881);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 19760);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 44752);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 73405);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 98380);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 116599);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 19761);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 19762);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 19763);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 19764);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 44753);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 73406);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 19765);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 44754);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 73407);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 44755);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 73408);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 19766);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 44756);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 73409);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 19767);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 44757);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 19768);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 19769);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 44758);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 73410);
									s.is_empty()
								}
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
			super::push(out, 4882);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 19770);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 44759);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 44760);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 73411);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 98381);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 44761);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 73412);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 19771);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 44762);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 73413);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 19772);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 44763);
							s.is_empty()
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 19773);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 44764);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 4883);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19774);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 44765);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 73414);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 98382);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 44766);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 73415);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 19775);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 44767);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 73416);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 98383);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 44768);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 73417);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 44769);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 73418);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 98384);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 44770);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 73419);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 98385);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 116600);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 73420);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 98386);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 116601);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => super::t28::n19882(s, out),
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 4884);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 19777);
					match s {
						[] => true,
						['ф', s @ ..] => super::t28::n45210(s, out),
						_ => false,
					}
				}
				['у', s @ ..] => super::t28::n19884(s, out),
				_ => false,
			}
		}
		['д', s @ ..] => super::t28::n4896(s, out),
		['е', s @ ..] => super::t28::n4897(s, out),
		['ж', s @ ..] => super::t28::n4898(s, out),
		['з', s @ ..] => super::t28::n4899(s, out),
		['и', s @ ..] => super::t28::n4900(s, out),
		['й', s @ ..] => super::t28::n4901(s, out),
		['к', s @ ..] => super::t28::n4902(s, out),
		['м', s @ ..] => super::t28::n4903(s, out),
		['о', s @ ..] => super::t28::n4904(s, out),
		['п', s @ ..] => super::t28::n4905(s, out),
		['р', s @ ..] => super::t28::n4906(s, out),
		['с', s @ ..] => super::t28::n4907(s, out),
		['т', s @ ..] => super::t28::n4908(s, out),
		['у', s @ ..] => super::t28::n4909(s, out),
		['ф', s @ ..] => super::t28::n4910(s, out),
		['х', s @ ..] => super::t28::n4911(s, out),
		['ч', s @ ..] => super::t28::n4912(s, out),
		['ш', s @ ..] => super::t28::n4913(s, out),
		['ы', s @ ..] => super::t29::n4914(s, out),
		['ь', s @ ..] => super::t29::n4915(s, out),
		['ю', s @ ..] => super::t29::n4916(s, out),
		['я', s @ ..] => super::t29::n4917(s, out),
		_ => false,
	}
}

/// Words ending -оя.
pub(crate) fn n592(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 591);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 4907);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 19886);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 4908);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 19887);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 4909);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 19888);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 4910);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 19889);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 4911);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 19890);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 44949);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 4912);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 19891);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 4913);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 4914);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 19892);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 4915);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 19893);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 44950);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 73593);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 98502);
											match s {
												[] => true,
												['я', s @ ..] => {
													super::push(out, 116664);
													match s {
														[] => true,
														['ь', s @ ..] => super::t29::n129477(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4916);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 19894);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 44951);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 73594);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 98503);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ря.
pub(crate) fn n593(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 592);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4917);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 19895);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 44952);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 44953);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 73595);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 98504);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 116665);
													match s {
														[] => true,
														['о', s @ ..] => super::t29::n129478(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 19896);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 44954);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 19897);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 44955);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 19898);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 4918);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 19899);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 44956);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 73596);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 98505);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 116666);
													match s {
														[] => true,
														['л', s @ ..] => super::t29::n129479(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4919);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 19900);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 44957);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 73597);
									s.is_empty()
								}
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
			super::push(out, 4920);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 19901);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 44958);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 73598);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 98506);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 116667);
													match s {
														[] => true,
														['о', s @ ..] => super::t29::n129480(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 19902);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 19903);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 44959);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 73599);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 98507);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 116668);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 19904);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 44960);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 73600);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 98508);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 44961);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 73601);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 44962);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 73602);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 98509);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 116669);
													s.is_empty()
												}
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
							super::push(out, 44963);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 19905);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 44964);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 73603);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 98510);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4921);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 19906);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 44965);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 73604);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 98511);
											s.is_empty()
										}
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
					super::push(out, 19907);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 44966);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 4922);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 19908);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 44967);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 44968);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73605);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 19909);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 44969);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 73606);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 19910);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 19911);
					match s {
						[] => true,
						['о', s @ ..] => super::t29::n45404(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => super::t29::n4934(s, out),
		['у', s @ ..] => super::t29::n4935(s, out),
		['ф', s @ ..] => super::t29::n4936(s, out),
		['ы', s @ ..] => super::t29::n4937(s, out),
		['ю', s @ ..] => super::t29::n4938(s, out),
		_ => false,
	}
}

/// Words ending -ся.
pub(crate) fn n594(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 593);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4928);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 19918);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 19919);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 44974);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 4929);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 19920);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 44975);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 73611);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 98515);
											match s {
												[] => true,
												['ч', s @ ..] => {
													super::push(out, 116671);
													match s {
														[] => true,
														['у', s @ ..] => super::t29::n129482(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ы', s @ ..] => {
											super::push(out, 98516);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 116672);
													match s {
														[] => true,
														['с', s @ ..] => super::t29::n129483(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['щ', s @ ..] => {
							super::push(out, 44976);
							match s {
								[] => true,
								['ю', s @ ..] => {
									super::push(out, 73612);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 98517);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 116673);
													match s {
														[] => true,
														['ы', s @ ..] => super::t29::n129484(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 19921);
					match s {
						[] => true,
						['щ', s @ ..] => {
							super::push(out, 44977);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 73613);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 98518);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 116674);
													match s {
														[] => true,
														['р', 'б', '\u{1}', s @ ..] => super::t29::n129485(s, out),
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
									super::push(out, 73614);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 98519);
											match s {
												[] => true,
												['ч', s @ ..] => {
													super::push(out, 116675);
													match s {
														[] => true,
														['а', s @ ..] => super::t29::n129486(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 19922);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 44978);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 44979);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 4930);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 19923);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 44980);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 73615);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 19924);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 44981);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 44982);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 73616);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 98520);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 116676);
													match s {
														[] => true,
														['к', s @ ..] => super::t29::n129487(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4931);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 19925);
					match s {
						[] => true,
						['щ', s @ ..] => {
							super::push(out, 44983);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 73617);
									match s {
										[] => true,
										['ч', s @ ..] => super::t29::n99593(s, out),
										_ => false,
									}
								}
								['ю', s @ ..] => super::t29::n74395(s, out),
								['я', s @ ..] => super::t29::n74396(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t29::n4943(s, out),
		['у', s @ ..] => super::t29::n4944(s, out),
		['ю', s @ ..] => super::t29::n4945(s, out),
		['я', s @ ..] => super::t29::n4946(s, out),
		_ => false,
	}
}

/// Words ending -тя.
pub(crate) fn n595(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 594);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4936);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 19936);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 19937);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 4937);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 19938);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 19939);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 4938);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 19940);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 19941);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 44993);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 19942);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 4939);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 19943);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 4940);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19944);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 44994);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 19945);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 44995);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 73631);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 19946);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 19947);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 44996);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 4941);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 19948);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 44997);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 73632);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 19949);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 4942);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 19950);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 44998);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 73633);
									s.is_empty()
								}
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
			super::push(out, 4943);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 19951);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 44999);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 4944);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 19952);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 45000);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 73634);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 19953);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уя.
pub(crate) fn n596(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 595);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 4945);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 19954);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 45001);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 4946);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 19955);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 45002);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 19956);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 45003);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 4947);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19957);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 45004);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 73635);
									s.is_empty()
								}
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
			super::push(out, 4948);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 19958);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 45005);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыя.
pub(crate) fn n597(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 596);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 4949);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ья.
pub(crate) fn n598(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 597);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 4950);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 19959);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 45006);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 73636);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 98530);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4951);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 19960);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 45007);
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
				['а', s @ ..] => {
					super::push(out, 19961);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 45008);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 45009);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 73637);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 45010);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 73638);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 98531);
											s.is_empty()
										}
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
					super::push(out, 19962);
					match s {
						[] => true,
						['ъ', s @ ..] => {
							super::push(out, 45011);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 73639);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 98532);
											s.is_empty()
										}
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
					super::push(out, 19963);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 45012);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 73640);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 19964);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 45013);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 45014);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 19965);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 45015);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73641);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 45016);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 73642);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 98533);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 116685);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4953);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 19966);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 45017);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 73643);
									s.is_empty()
								}
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
			super::push(out, 4954);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19967);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45018);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 4955);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19968);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 45019);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73644);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 73645);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 98534);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 116686);
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 98535);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 116687);
													match s {
														[] => true,
														['а', s @ ..] => super::t29::n129493(s, out),
														_ => false,
													}
												}
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
							super::push(out, 45020);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 73646);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 98536);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 45021);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 73647);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 98537);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 116688);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 19969);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 45022);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 19970);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45023);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 45024);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 45025);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 19971);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 45026);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73648);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 45027);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 73649);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 98538);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 116689);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 19972);
					match s {
						[] => true,
						['г', s @ ..] => super::t29::n45462(s, out),
						['т', s @ ..] => super::t29::n45463(s, out),
						_ => false,
					}
				}
				['ы', s @ ..] => super::t29::n20079(s, out),
				_ => false,
			}
		}
		['м', s @ ..] => super::t29::n4967(s, out),
		['н', s @ ..] => super::t29::n4968(s, out),
		['п', s @ ..] => super::t29::n4969(s, out),
		['р', s @ ..] => super::t29::n4970(s, out),
		['с', s @ ..] => super::t29::n4971(s, out),
		['т', s @ ..] => super::t29::n4972(s, out),
		['ф', s @ ..] => super::t29::n4973(s, out),
		['х', s @ ..] => super::t29::n4974(s, out),
		['ч', s @ ..] => super::t29::n4975(s, out),
		['ш', s @ ..] => super::t29::n4976(s, out),
		_ => false,
	}
}

/// Words ending -яя.
pub(crate) fn n599(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 598);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4966);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 4967);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 20007);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 45099);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 73724);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 45100);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 73725);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 98587);
											s.is_empty()
										}
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
					super::push(out, 20008);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 45101);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 73726);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 45102);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 73727);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 98588);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 116712);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 20009);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 45103);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 73728);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 98589);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 116713);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еба.
pub(crate) fn n605(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 604);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4984);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 20032);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45117);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 4985);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20033);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 45118);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 73737);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 98598);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 116714);
													match s {
														[] => true,
														['д', s @ ..] => super::t29::n129501(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4986);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20034);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 4987);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20035);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 45119);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 20036);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 20037);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 45120);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 4988);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20038);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 45121);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 73738);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 98599);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 116715);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4989);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 20039);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 45122);
							s.is_empty()
						}
						['я', s @ ..] => {
							super::push(out, 45123);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73739);
									s.is_empty()
								}
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
			super::push(out, 4990);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 4991);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 20040);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ъ', s @ ..] => {
			super::push(out, 4992);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 20041);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45124);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 73740);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -орожба.
pub(crate) fn n45560(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 45126);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 73741);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 73742);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 98600);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ужба.
pub(crate) fn n5005(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4994);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 20044);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зба.
pub(crate) fn n607(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 606);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 4995);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20045);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 20046);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 45127);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иба.
pub(crate) fn n608(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 607);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4996);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 4997);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 4998);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20047);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 4999);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 20048);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45128);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 5000);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20049);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45129);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 5001);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 5002);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 20050);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йба.
pub(crate) fn n609(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 608);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5003);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 20051);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5004);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 20052);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45130);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лба.
pub(crate) fn n610(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 609);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5005);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 20053);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45131);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5006);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 20054);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45132);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 73743);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 98601);
											s.is_empty()
										}
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
					super::push(out, 20055);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 20056);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 45133);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мба.
pub(crate) fn n611(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 610);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5007);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 20057);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 20058);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45134);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73744);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 20059);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 20060);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45135);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 20061);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5008);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 20062);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45136);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5009);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20063);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 20064);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 20065);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45137);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 73745);
									s.is_empty()
								}
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
			super::push(out, 5010);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 20066);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 45138);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 73746);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 98602);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 116716);
													match s {
														[] => true,
														['с', s @ ..] => super::t29::n129502(s, out),
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
									super::push(out, 73747);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 98603);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 116717);
													match s {
														[] => true,
														['б', s @ ..] => super::t29::n129503(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 20067);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 20068);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 45139);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 20069);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 5011);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 20070);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 45140);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 73748);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 98604);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 116718);
													match s {
														[] => true,
														['в', s @ ..] => super::t29::n129504(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 5012);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оба.
pub(crate) fn n612(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 611);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 5013);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20071);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 20072);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 45141);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 73749);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 98605);
											s.is_empty()
										}
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
					super::push(out, 20073);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 45142);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 73750);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 98606);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5014);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20074);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 45143);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73751);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 45144);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73752);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 20075);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 45145);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 73753);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 98607);
											s.is_empty()
										}
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
					super::push(out, 20076);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 45146);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73754);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 20077);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 45147);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 73755);
									s.is_empty()
								}
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
			super::push(out, 5015);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20078);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 45148);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73756);
									s.is_empty()
								}
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
			super::push(out, 5016);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 20079);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 45149);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 20080);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 45150);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 73757);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 98608);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 116719);
													match s {
														[] => true,
														['л', s @ ..] => super::t29::n129505(s, out),
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
									super::push(out, 73758);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 98609);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 116720);
													match s {
														[] => true,
														['б', s @ ..] => super::t29::n129506(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 5017);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20081);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 45151);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73759);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 20082);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 45152);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 73760);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 20083);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 5018);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 20084);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45153);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 73761);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 98610);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5019);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 20085);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 20086);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 45154);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73762);
									s.is_empty()
								}
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
			super::push(out, 5020);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20087);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45155);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 45156);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 73763);
									s.is_empty()
								}
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
			super::push(out, 5021);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 20088);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45157);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73764);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 5022);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 20089);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 45158);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рба.
pub(crate) fn n613(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 612);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5023);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 20090);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 45159);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 73765);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 98611);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 116721);
													match s {
														[] => true,
														['ж', s @ ..] => super::t29::n129507(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 5024);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 20091);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 20092);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45160);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5025);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 20093);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 5026);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 20094);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45161);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сба.
pub(crate) fn n614(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 613);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5027);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 20095);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5028);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 20096);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45162);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тба.
pub(crate) fn n615(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 614);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 5029);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 20097);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уба.
pub(crate) fn n616(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 615);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 5030);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20098);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 20099);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45163);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 5031);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20100);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 20101);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 45164);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 5032);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 20102);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 45165);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 73766);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 98612);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 116722);
													match s {
														[] => true,
														['г', s @ ..] => super::t29::n129508(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 20103);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 45166);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 20104);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 20105);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45167);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 20106);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45168);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 5033);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20107);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 45169);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73767);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 45170);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 73768);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 20108);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 45171);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20109);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 45172);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 5034);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20110);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 45173);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20111);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 45174);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73769);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 20112);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 45175);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 73770);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 98613);
											match s {
												[] => true,
												['ы', s @ ..] => {
													super::push(out, 116723);
													match s {
														[] => true,
														['р', s @ ..] => super::t29::n129509(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['л', s @ ..] => {
											super::push(out, 98614);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 116724);
													match s {
														[] => true,
														['и', s @ ..] => super::t29::n129510(s, out),
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
									super::push(out, 73771);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 98615);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 116725);
													match s {
														[] => true,
														['а', s @ ..] => super::t29::n129511(s, out),
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
									super::push(out, 73772);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 98616);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 116726);
													match s {
														[] => true,
														['й', s @ ..] => super::t29::n129512(s, out),
														_ => false,
													}
												}
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
							super::push(out, 45176);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 73773);
									s.is_empty()
								}
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
			super::push(out, 5035);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 5036);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чба.
pub(crate) fn n617(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 616);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 5037);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20113);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45177);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шба.
pub(crate) fn n618(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 617);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 5038);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20114);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 45178);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыба.
pub(crate) fn n619(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 618);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 5039);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 20115);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45179);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 73774);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 98617);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5040);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 20116);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 20117);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 45180);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 73775);
									s.is_empty()
								}
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
			super::push(out, 5041);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 20118);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45181);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 73776);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 73777);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 98618);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ж', s @ ..] => {
							super::push(out, 45182);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 73778);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 98619);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['я', s @ ..] => {
							super::push(out, 45183);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 73779);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 98620);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 116727);
													match s {
														[] => true,
														['с', s @ ..] => super::t29::n129513(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 20119);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 45184);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 73780);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 98621);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьба.
pub(crate) fn n620(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 619);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 5042);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20120);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 45185);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 73781);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 45186);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 73782);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 98622);
											match s {
												[] => true,
												['й', s @ ..] => {
													super::push(out, 116728);
													match s {
														[] => true,
														['е', s @ ..] => super::t29::n129514(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 20121);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 20122);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 45187);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 73783);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 45188);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20123);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 45189);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 20124);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 45190);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 45191);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 5043);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 20125);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 45192);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 20126);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 45193);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 73784);
									s.is_empty()
								}
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
			super::push(out, 5044);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20127);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45194);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 45195);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 73785);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 98623);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 116729);
													s.is_empty()
												}
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
							super::push(out, 45196);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 73786);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 98624);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 45197);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 20128);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 45198);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 73787);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 98625);
											s.is_empty()
										}
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
					super::push(out, 20129);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 45199);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 73788);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 98626);
											s.is_empty()
										}
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
					super::push(out, 20130);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 45200);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 20131);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 45201);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 45202);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 5045);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20132);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 45203);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 5046);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20133);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 45204);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 20134);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 5047);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20135);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 45205);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 45206);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 73789);
									s.is_empty()
								}
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
			super::push(out, 5048);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20136);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 45207);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73790);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 20137);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 45208);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 73791);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 98627);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 45209);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 73792);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 98628);
											s.is_empty()
										}
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
					super::push(out, 20138);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 45210);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 73793);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 20139);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45211);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => super::t29::n20246(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юба.
pub(crate) fn n621(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 620);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 5049);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20141);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 5050);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -яба.
pub(crate) fn n622(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 621);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 5051);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 20142);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 45213);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 73795);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 98629);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 116730);
													match s {
														[] => true,
														['о', s @ ..] => super::t29::n129515(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 5052);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 20143);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -экс-глава.
pub(crate) fn n117990(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 116731);
	s.is_empty()
}

/// Words ending -балаклава.
pub(crate) fn n117991(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 116732);
	s.is_empty()
}

/// Words ending -слава.
pub(crate) fn n20259(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20153);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 45224);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 45225);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 45226);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 73801);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 98634);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 116733);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 128166);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 134898);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 45227);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 73802);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -улава.
pub(crate) fn n20260(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20154);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 45228);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 73803);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 98635);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хлава.
pub(crate) fn n20261(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20155);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 45229);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 73804);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цлава.
pub(crate) fn n20262(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20156);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 45230);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 73805);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 98636);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мава.
pub(crate) fn n5070(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5059);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 20157);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 45231);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 73806);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 98637);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нава.
pub(crate) fn n5071(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5060);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 20158);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 45232);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пава.
pub(crate) fn n5072(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5061);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 20159);
			s.is_empty()
		}
		['у', 'к', '\u{1}', s @ ..] => {
			super::push(out, 20160);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рава.
pub(crate) fn n5073(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5062);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 20161);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 45233);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 73807);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 20162);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 45234);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 73808);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 98638);
									s.is_empty()
								}
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
			super::push(out, 20163);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 45235);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 20164);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 20165);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 45236);
					s.is_empty()
				}
				['е', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 73809);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 98639);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 116734);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 128167);
													s.is_empty()
												}
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
							super::push(out, 73810);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 45237);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 45238);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 73811);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 98640);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 116735);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 128168);
													s.is_empty()
												}
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
							super::push(out, 73812);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 98641);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 116736);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 128169);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 45239);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 45240);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 20166);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 45241);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 45242);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 73813);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 98642);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 116737);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 128170);
													match s {
														[] => true,
														['а', s @ ..] => super::t29::n136304(s, out),
														_ => false,
													}
												}
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
							super::push(out, 73814);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 98643);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 116738);
											s.is_empty()
										}
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
					super::push(out, 45243);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 20167);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 45244);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 73815);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сава.
pub(crate) fn n5074(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5063);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 20168);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 20169);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 45245);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тава.
pub(crate) fn n5075(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5064);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 20170);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 20171);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 45246);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уава.
pub(crate) fn n5076(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5065);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 20172);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шава.
pub(crate) fn n5078(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5067);
	s.is_empty()
}

/// Words ending -бва.
pub(crate) fn n624(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 623);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 5068);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 20173);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 45247);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 73816);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 98644);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вва.
pub(crate) fn n625(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 624);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5069);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20174);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 20175);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гва.
pub(crate) fn n626(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 625);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 5070);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 20176);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 45248);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 73817);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 73818);
									s.is_empty()
								}
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
			super::push(out, 5071);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 20177);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45249);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -два.
pub(crate) fn n627(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 626);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 5072);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 20178);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45250);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 5073);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20179);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 45251);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 73819);
									s.is_empty()
								}
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
			super::push(out, 5074);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20180);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 45252);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73820);
									s.is_empty()
								}
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
			super::push(out, 5075);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20181);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 45253);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ева.
pub(crate) fn n628(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 627);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 5076);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 20182);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 45254);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 5077);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 5078);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 20183);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 45255);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73821);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20184);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 45256);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 73822);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 98645);
											s.is_empty()
										}
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
					super::push(out, 20185);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45257);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 5079);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20186);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 20187);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 45258);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 20188);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 45259);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 73823);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 98646);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 116739);
													match s {
														[] => true,
														['д', s @ ..] => super::t29::n129521(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 20189);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 45260);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 20190);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 45261);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20191);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 45262);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 20192);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 45263);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 73824);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 20193);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 45264);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 73825);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 98647);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 116740);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 5080);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 5081);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 20194);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 45265);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 20195);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 45266);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 5082);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 20196);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 45267);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 73826);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 98648);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 116741);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 20197);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 45268);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73827);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зва.
pub(crate) fn n629(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 628);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 5083);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 20198);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 5084);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ива.
pub(crate) fn n630(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 629);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 5085);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 20199);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 20200);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 45269);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 5086);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20201);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 45270);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20202);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 45271);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 5087);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 5088);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20203);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 45272);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73828);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 45273);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20204);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 20205);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 5089);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 5090);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 5091);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 5092);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20206);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 20207);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 45274);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73829);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 20208);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 45275);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 5093);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 20209);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45276);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 73830);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 98649);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 116742);
													match s {
														[] => true,
														['п', s @ ..] => super::t29::n129522(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 5094);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -йва.
pub(crate) fn n631(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 630);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5095);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20210);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 20211);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5096);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 20212);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 5097);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 20213);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45277);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ква.
pub(crate) fn n632(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 631);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5098);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 20214);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 45278);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73831);
									s.is_empty()
								}
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
			super::push(out, 5099);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 20215);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 45279);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 73832);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 98650);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5100);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 5101);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 20216);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 45280);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20217);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 45281);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 5102);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20218);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 45282);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 5103);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 20219);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 5104);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 20220);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 5105);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 20221);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 5106);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 5107);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 20222);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 45283);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 45284);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лва.
pub(crate) fn n633(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 632);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5108);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 20223);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45285);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5109);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 20224);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мва.
pub(crate) fn n634(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 633);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 5110);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нва.
pub(crate) fn n635(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 634);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5111);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 20225);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45286);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 20226);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45287);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 73833);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ова.
pub(crate) fn n636(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 635);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 5112);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20227);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 5113);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 20228);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 45288);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 5114);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 20229);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 20230);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 45289);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73834);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 20231);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 45290);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 73835);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 20232);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 45291);
							s.is_empty()
						}
						['я', s @ ..] => {
							super::push(out, 45292);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 20233);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 45293);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 73836);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 98651);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5115);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20234);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 5116);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 20235);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 45294);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 73837);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 20236);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 45295);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73838);
									s.is_empty()
								}
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
			super::push(out, 5117);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20237);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 45296);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73839);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 73840);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 98652);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 116743);
													match s {
														[] => true,
														['о', s @ ..] => super::t29::n129523(s, out),
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
									super::push(out, 73841);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 98653);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 116744);
													match s {
														[] => true,
														['п', s @ ..] => super::t29::n129524(s, out),
														_ => false,
													}
												}
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
							super::push(out, 45297);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73842);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 20238);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45298);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 45299);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 73843);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 98654);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 116745);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 5118);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 5119);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20239);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 45300);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 73844);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 98655);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 45301);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 20240);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 45302);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 20241);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 45303);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 73845);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 98656);
											match s {
												[] => true,
												['ы', s @ ..] => {
													super::push(out, 116746);
													match s {
														[] => true,
														['з', s @ ..] => super::t29::n129525(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 5120);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 20242);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 45304);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 73846);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 20243);
					match s {
						[] => true,
						['н', s @ ..] => super::t29::n45740(s, out),
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 20244);
					s.is_empty()
				}
				['о', s @ ..] => super::t29::n20351(s, out),
				['т', s @ ..] => super::t29::n20352(s, out),
				_ => false,
			}
		}
		['с', s @ ..] => super::t29::n5132(s, out),
		['х', s @ ..] => super::t29::n5133(s, out),
		_ => false,
	}
}

/// Words ending -рва.
pub(crate) fn n637(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 636);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5123);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 20248);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45309);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 20249);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 20250);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45310);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 73850);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 98659);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5124);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 20251);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 20252);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 45311);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 73851);
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
							super::push(out, 45312);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 45313);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 73852);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 20253);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 45314);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 20254);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 45315);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 20255);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5125);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 20256);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 45316);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 20257);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 45317);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 5126);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 20258);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 5127);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 20259);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 5128);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 20260);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сва.
pub(crate) fn n638(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 637);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 5129);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 20261);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 45318);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73853);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тва.
pub(crate) fn n639(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 638);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5130);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 20262);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 20263);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 45319);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73854);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 45320);
							s.is_empty()
						}
						['ж', s @ ..] => {
							super::push(out, 45321);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73855);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 20264);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 45322);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 73856);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 98660);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 116749);
													match s {
														[] => true,
														['б', s @ ..] => super::t29::n129527(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 5131);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 20265);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45323);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 20266);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5132);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 20267);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 20268);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45324);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 45325);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 73857);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 20269);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 45326);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5133);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 20270);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45327);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 20271);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 20272);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 45328);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 5134);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 20273);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 45329);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73858);
									s.is_empty()
								}
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
			super::push(out, 5135);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20274);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 45330);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 20275);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 45331);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 73859);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 98661);
											s.is_empty()
										}
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
					super::push(out, 20276);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 45332);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 73860);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 20277);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 45333);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 73861);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 98662);
											s.is_empty()
										}
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
					super::push(out, 20278);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 45334);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 20279);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45335);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 73862);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 98663);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 116750);
													match s {
														[] => true,
														['п', '\u{1}', s @ ..] => super::t29::n129528(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', '\u{1}', s @ ..] => {
					super::push(out, 20280);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 5136);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 20281);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 45336);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ува.
pub(crate) fn n640(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 639);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 5137);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20282);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 5138);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 20283);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хва.
pub(crate) fn n641(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 640);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 5139);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 20284);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45337);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 5140);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20285);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 45338);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 5141);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 20286);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чва.
pub(crate) fn n642(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 641);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 5142);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 20287);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шва.
pub(crate) fn n643(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 642);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 5143);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыва.
pub(crate) fn n644(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 643);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 5144);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 5145);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 5146);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20288);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьва.
pub(crate) fn n645(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 644);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 5147);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20289);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 20290);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 20291);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 45339);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73863);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 20292);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 5148);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 20293);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 45340);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эва.
pub(crate) fn n646(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 645);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5149);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 5150);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ява.
pub(crate) fn n647(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 646);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5151);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 5152);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20294);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 45341);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 5153);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20295);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 45342);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 20296);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45343);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 73864);
									s.is_empty()
								}
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
			super::push(out, 5154);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20297);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 45344);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 20298);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 45345);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 20299);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45346);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 73865);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 98664);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5155);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 20300);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 45347);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 73866);
									s.is_empty()
								}
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
			super::push(out, 5156);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 20301);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45348);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ъ', s @ ..] => {
			super::push(out, 5157);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -спецблага.
pub(crate) fn n118010(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 116751);
	s.is_empty()
}

/// Words ending -гипаллага.
pub(crate) fn n118011(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 116752);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 128179);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -металлага.
pub(crate) fn n118012(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 116753);
	s.is_empty()
}

/// Words ending -крав-мага.
pub(crate) fn n118013(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 116754);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 128180);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -льпага.
pub(crate) fn n45797(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 45362);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 73875);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 98670);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рага.
pub(crate) fn n5176(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5165);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 20319);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 45363);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 20320);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 20321);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 20322);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 45364);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 73876);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 20323);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чага.
pub(crate) fn n5179(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5168);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 20324);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 45365);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 73877);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 98671);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 116755);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щага.
pub(crate) fn n5180(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5169);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 20325);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 45366);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гга.
pub(crate) fn n649(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 648);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5170);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 20326);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45367);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5171);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 20327);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5172);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 20328);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 45368);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 73878);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 98672);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 116756);
													match s {
														[] => true,
														['о', s @ ..] => super::t29::n129531(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ега.
pub(crate) fn n650(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 649);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 5173);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20329);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 5174);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 20330);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 5175);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20331);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 5176);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 20332);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 45369);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 20333);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 45370);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 73879);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 20334);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45371);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 5177);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20335);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 5178);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20336);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 20337);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5179);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 20338);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 5180);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 5181);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 20339);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 45372);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 73880);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 98673);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 116757);
													s.is_empty()
												}
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
							super::push(out, 45373);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 5182);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20340);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 45374);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 73881);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 98674);
											s.is_empty()
										}
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
					super::push(out, 20341);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45375);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 45376);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 73882);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 98675);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 45377);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 73883);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 98676);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 116758);
													match s {
														[] => true,
														['и', s @ ..] => super::t29::n129532(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 5183);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20342);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 45378);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73884);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зга.
pub(crate) fn n651(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 650);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 5184);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 20343);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5185);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 20344);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 5186);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 20345);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 20346);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 5187);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 20347);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 45379);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 5188);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 20348);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 45380);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 73885);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 20349);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 45381);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73886);
									s.is_empty()
								}
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
			super::push(out, 5189);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 20350);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 45382);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ига.
pub(crate) fn n652(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 651);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 5190);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20351);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 45383);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73887);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 20352);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 5191);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 20353);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 45384);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73888);
									s.is_empty()
								}
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
			super::push(out, 5192);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20354);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 5193);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20355);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 45385);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73889);
									s.is_empty()
								}
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
			super::push(out, 5194);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 20356);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 45386);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 73890);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 98677);
											s.is_empty()
										}
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
					super::push(out, 20357);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 45387);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73891);
									s.is_empty()
								}
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
			super::push(out, 5195);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 5196);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20358);
					match s {
						[] => true,
						['к', '\u{1}', s @ ..] => {
							super::push(out, 45388);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 5197);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 20359);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 5198);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 20360);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 45389);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 73892);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 98678);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 116759);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t29::n129533(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 5199);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 20361);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 45390);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йга.
pub(crate) fn n653(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 652);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5200);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 20362);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 45391);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 73893);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 98679);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 116760);
													match s {
														[] => true,
														['а', s @ ..] => super::t29::n129534(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 20363);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 20364);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лга.
pub(crate) fn n654(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 653);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 5201);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 20365);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45392);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 73894);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 45393);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73895);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 20366);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 45394);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 73896);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 20367);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45395);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 5202);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 20368);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 45396);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 73897);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 98680);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мга.
pub(crate) fn n655(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 654);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5203);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20369);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 20370);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45397);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5204);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 20371);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5205);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 20372);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нга.
pub(crate) fn n656(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 655);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5206);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 20373);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45398);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 73898);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 98681);
											s.is_empty()
										}
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
					super::push(out, 20374);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 20375);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 20376);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45399);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 73899);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 98682);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 45400);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5207);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 20377);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 45401);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 73900);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 98683);
											s.is_empty()
										}
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
					super::push(out, 20378);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 45402);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 20379);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 45403);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 73901);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 98684);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5208);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20380);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 20381);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 20382);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45404);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 20383);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 20384);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 45405);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 73902);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 20385);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 45406);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 73903);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 98685);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 45407);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 73904);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 20386);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 45408);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 73905);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 45409);
							match s {
								[] => true,
								['к', '\u{1}', s @ ..] => {
									super::push(out, 73906);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 20387);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 45410);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 73907);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 20388);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45411);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5209);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 20389);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 20390);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 45412);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 73908);
									s.is_empty()
								}
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
			super::push(out, 5210);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 20391);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45413);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 5211);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 20392);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45414);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 5212);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20393);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 5213);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 20394);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 45415);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

