use flexion::Path;

/// Words ending -тельница.
pub(crate) fn n104877(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 103782);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 119772);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 129717);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 135601);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 139030);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 139031);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 141015);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 142180);
													match s {
														[] => true,
														['р', s @ ..] => super::t54::n144290(s, out),
														_ => false,
													}
												}
												['о', s @ ..] => {
													super::push(out, 142181);
													match s {
														[] => true,
														['х', s @ ..] => super::t54::n144291(s, out),
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
									super::push(out, 139032);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 141016);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 135602);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 139033);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 141017);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 135603);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 135604);
							match s {
								[] => true,
								['в', 'т', s @ ..] => {
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 141018);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 142182);
													match s {
														[] => true,
														['ж', s @ ..] => super::t54::n144292(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 141019);
											s.is_empty()
										}
										_ => false,
									}
								}
								['д', s @ ..] => {
									super::push(out, 139034);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 141020);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 142183);
													match s {
														[] => true,
														['с', s @ ..] => super::t54::n144293(s, out),
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
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 141021);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 142184);
													match s {
														[] => true,
														['б', s @ ..] => super::t54::n144294(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ь', s @ ..] => super::t54::n142467(s, out),
										_ => false,
									}
								}
								['к', s @ ..] => super::t54::n140472(s, out),
								['н', s @ ..] => super::t54::n140473(s, out),
								['р', s @ ..] => super::t54::n140474(s, out),
								_ => false,
							}
						}
						['ы', s @ ..] => super::t54::n137011(s, out),
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 129718);
					s.is_empty()
				}
				['д', s @ ..] => super::t54::n131071(s, out),
				['ж', s @ ..] => super::t54::n131072(s, out),
				['к', s @ ..] => super::t54::n131073(s, out),
				['л', s @ ..] => super::t54::n131074(s, out),
				['р', s @ ..] => super::t54::n131075(s, out),
				['т', s @ ..] => super::t54::n131076(s, out),
				['ш', s @ ..] => super::t54::n131077(s, out),
				['щ', s @ ..] => super::t54::n131078(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t54::n121036(s, out),
		['с', s @ ..] => super::t54::n121037(s, out),
		['я', s @ ..] => super::t54::n121038(s, out),
		_ => false,
	}
}

/// The word оконтуривание.
pub(crate) fn n143661(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142210);
	s.is_empty()
}

/// Words ending -упорядочивание.
pub(crate) fn n143662(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142211);
	s.is_empty()
}

/// Words ending -поспешествование.
pub(crate) fn n144653(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143200);
	s.is_empty()
}

/// Words ending -триаршествование.
pub(crate) fn n144654(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143201);
	s.is_empty()
}

/// The word долженствование.
pub(crate) fn n144655(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143202);
	s.is_empty()
}

/// Words ending -ман-исследование.
pub(crate) fn n143674(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142223);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 142863);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 143203);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -израсходование.
pub(crate) fn n143675(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142224);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 142864);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ирование.
pub(crate) fn n105879(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 104771);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 120416);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 130106);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 135825);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 139160);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 141092);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 142226);
													s.is_empty()
												}
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
							super::push(out, 135826);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 139161);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 141093);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 141094);
											s.is_empty()
										}
										_ => false,
									}
								}
								['л', s @ ..] => {
									super::push(out, 139162);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 141095);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 142227);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 130107);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 120417);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 130108);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 135827);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 139163);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 141096);
											s.is_empty()
										}
										['ы', s @ ..] => {
											super::push(out, 141097);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 142228);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t54::n144319(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 130109);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 120418);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 120419);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 130110);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 130111);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 130112);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 135828);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 139164);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 141098);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 142229);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['м', s @ ..] => {
									super::push(out, 139165);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 141099);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 142230);
													s.is_empty()
												}
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
							super::push(out, 135829);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 135830);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 135831);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 139166);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 141100);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 142231);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 130113);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 130114);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 135832);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 139167);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 141101);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 142232);
													match s {
														[] => true,
														['б', s @ ..] => super::t54::n144320(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 120420);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 130115);
					s.is_empty()
				}
				['а', s @ ..] => {
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 135833);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 135834);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 139168);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 141102);
											s.is_empty()
										}
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
					super::push(out, 130116);
					match s {
						[] => true,
						['а', 'р', 'а', s @ ..] => {
							super::push(out, 135835);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 139169);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 141103);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 142233);
													match s {
														[] => true,
														['п', s @ ..] => super::t54::n144321(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 120421);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 130117);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 135836);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 139170);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 141104);
											s.is_empty()
										}
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
					super::push(out, 130118);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 130119);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 135837);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => super::t54::n121689(s, out),
		['л', s @ ..] => super::t54::n121690(s, out),
		['м', s @ ..] => super::t54::n121691(s, out),
		['н', s @ ..] => super::t54::n121692(s, out),
		['п', s @ ..] => super::t54::n121693(s, out),
		['р', s @ ..] => {
			super::push(out, 120427);
			match s {
				[] => true,
				['ф', s @ ..] => super::t54::n131499(s, out),
				_ => false,
			}
		}
		['с', s @ ..] => super::t54::n121695(s, out),
		['т', s @ ..] => super::t54::n121696(s, out),
		['у', s @ ..] => super::t54::n121697(s, out),
		['ф', s @ ..] => super::t54::n121698(s, out),
		['ц', s @ ..] => super::t54::n121699(s, out),
		['ш', s @ ..] => super::t54::n121700(s, out),
		_ => false,
	}
}

/// Words ending -ина-водоисточник.
pub(crate) fn n144684(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143231);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 143384);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word рак-отшельник.
pub(crate) fn n143799(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142347);
	s.is_empty()
}

/// Words ending -инец-отшельник.
pub(crate) fn n143800(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142348);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 142927);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 143235);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 143386);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ат-пакетировщик.
pub(crate) fn n144405(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142952);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 143251);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 143395);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 143459);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word головничество.
pub(crate) fn n143879(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142427);
	s.is_empty()
}

/// Words ending -дничество.
pub(crate) fn n125267(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123983);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 132077);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 136874);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 139752);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 141445);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 142428);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 142966);
													match s {
														[] => true,
														['н', s @ ..] => super::t54::n144711(s, out),
														_ => false,
													}
												}
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
							super::push(out, 139753);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 136875);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 132078);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 136876);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 139754);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 141446);
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
							super::push(out, 139755);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 141447);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 142429);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 142967);
													s.is_empty()
												}
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
							super::push(out, 139756);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 141448);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 136877);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 132079);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 136878);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 136879);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 139757);
							match s {
								[] => true,
								['г', '\u{1}', s @ ..] => {
									super::push(out, 141449);
									s.is_empty()
								}
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
			super::push(out, 132080);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 136880);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еничество.
pub(crate) fn n125268(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123984);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 132081);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 136881);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 139758);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 141450);
									s.is_empty()
								}
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
			super::push(out, 132082);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 136882);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 139759);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 139760);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зничество.
pub(crate) fn n125270(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123986);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 132083);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 136883);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 139761);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 132084);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 136884);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 139762);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 141451);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 142430);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ойничество.
pub(crate) fn n133445(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132085);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 136885);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 139763);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 141452);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нничество.
pub(crate) fn n125273(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123989);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['д', 'д', s @ ..] => {
					super::push(out, 136886);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 139764);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 141453);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 142431);
											s.is_empty()
										}
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
					super::push(out, 136887);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 132086);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 136888);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 139765);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 141454);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 139766);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 141455);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 142432);
											s.is_empty()
										}
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
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 139767);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 139768);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 141456);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 142433);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 142968);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 136889);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 136890);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 139769);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 141457);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 136891);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 132087);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 136892);
					match s {
						[] => true,
						['к', 'о', 'п', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 141458);
									s.is_empty()
								}
								['о', 'л', s @ ..] => {
									super::push(out, 141459);
									match s {
										[] => true,
										['о', 'в', s @ ..] => {
											super::push(out, 142434);
											match s {
												[] => true,
												['я', s @ ..] => {
													super::push(out, 142969);
													match s {
														[] => true,
														['ь', s @ ..] => super::t54::n144712(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пничество.
pub(crate) fn n125274(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123990);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 132088);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 136893);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 132089);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 136894);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рничество.
pub(crate) fn n125275(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123991);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 132090);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 132091);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 136895);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 139770);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 132092);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 136896);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 139771);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 141460);
									s.is_empty()
								}
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
			super::push(out, 132093);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 132094);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 136897);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 139772);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 141461);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 142435);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сничество.
pub(crate) fn n125276(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123992);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 132095);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 136898);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 139773);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 132096);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 136899);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 139774);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 141462);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 142436);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тничество.
pub(crate) fn n125277(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123993);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 132097);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 132098);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 132099);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 136900);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 139775);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 141463);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 142437);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 142970);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 132100);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 136901);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 139776);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 141464);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 142438);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 142971);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 136902);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 139777);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 136903);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 139778);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 136904);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 139779);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 141465);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 142439);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 142972);
													s.is_empty()
												}
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
							super::push(out, 139780);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 141466);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 142440);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 142973);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 132101);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 136905);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 139781);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 141467);
									s.is_empty()
								}
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
			super::push(out, 132102);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чничество.
pub(crate) fn n125278(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123994);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 132103);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 136906);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 139782);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 141468);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 136907);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 139783);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 141469);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 136908);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 139784);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 136909);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 139785);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 136910);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 139786);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 136911);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 139787);
							s.is_empty()
						}
						['я', s @ ..] => {
							super::push(out, 139788);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 141470);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 142441);
											s.is_empty()
										}
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
					super::push(out, 136912);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 139789);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шничество.
pub(crate) fn n125279(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123995);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 132104);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 136913);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 139790);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 141471);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 142442);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 142974);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьничество.
pub(crate) fn n125281(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123997);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 132105);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ричество.
pub(crate) fn n111740(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 110571);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 123998);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 132106);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 123999);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 132107);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 136914);
							match s {
								[] => true,
								['л', 'э', 'о', s @ ..] => {
									super::push(out, 139791);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 132108);
					match s {
						[] => true,
						['е', s @ ..] => {
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 139792);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 139793);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 141472);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 136915);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тичество.
pub(crate) fn n111741(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 110572);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 124000);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 132109);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 136916);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 139794);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щичество.
pub(crate) fn n111742(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 110573);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 124001);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 132110);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 136917);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 139795);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 141473);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 142443);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нес-партнерство.
pub(crate) fn n143900(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142448);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 142976);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 143261);
					match s {
						[] => true,
						['б', '\u{1}', s @ ..] => {
							super::push(out, 143397);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -автоантизапотеватель.
pub(crate) fn n144504(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143051);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 143295);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сопереживатель.
pub(crate) fn n144021(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142569);
	s.is_empty()
}

/// Words ending -ласкиватель.
pub(crate) fn n139012(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137592);
	match s {
		[] => true,
		['о', 'п', 'о', '-', s @ ..] => {
			super::push(out, 140145);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 141681);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 142570);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 143052);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 143296);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 143414);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юкиватель.
pub(crate) fn n127597(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126297);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 133375);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 137594);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 140146);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ливатель.
pub(crate) fn n115723(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 114509);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 126298);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 133376);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 137595);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 126299);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 133377);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 137596);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 140147);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 133378);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 137597);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 140148);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 141682);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 142571);
													match s {
														[] => true,
														['р', s @ ..] => super::t54::n144506(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 126300);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 133379);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 137598);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 140149);
									s.is_empty()
								}
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
			super::push(out, 126301);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 133380);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 137599);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 140150);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 133381);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 137600);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 133382);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 137601);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 126302);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 126303);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ниватель.
pub(crate) fn n115724(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 114510);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 126304);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 133383);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 137602);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 140151);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 141683);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 142572);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 126305);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 133384);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 137603);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -риватель.
pub(crate) fn n115725(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 114511);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 126306);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 133385);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 126307);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ачиватель.
pub(crate) fn n127608(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126308);
	s.is_empty()
}

/// Words ending -шиватель.
pub(crate) fn n115727(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 114513);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 126309);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 133386);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 126310);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 133387);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 137604);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -т-исследователь.
pub(crate) fn n144025(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142573);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 143054);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 143297);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 143055);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 143298);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word преобразователь.
pub(crate) fn n144026(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142574);
	s.is_empty()
}

/// Words ending -опреобразователь.
pub(crate) fn n144027(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142575);
	s.is_empty()
}

/// Words ending -кообразователь.
pub(crate) fn n144028(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 143056);
			match s {
				[] => true,
				['е', 'л', 'п', '\u{1}', s @ ..] => {
					super::push(out, 143299);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 143057);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 143300);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 143415);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нообразователь.
pub(crate) fn n144029(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142576);
	s.is_empty()
}

/// Words ending -сообразователь.
pub(crate) fn n144030(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142577);
	s.is_empty()
}

/// Words ending -пользователь.
pub(crate) fn n141599(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 141688);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 142578);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 143058);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 143301);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 143416);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 143469);
													match s {
														[] => true,
														['т', s @ ..] => super::t54::n144946(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 141689);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кователь.
pub(crate) fn n115732(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 114518);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 126317);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 133394);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 137613);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 133395);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 137614);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 140156);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 140157);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 140158);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 141690);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 126318);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 133396);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 137615);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лователь.
pub(crate) fn n115733(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 114519);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 126319);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 133397);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 137616);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нователь.
pub(crate) fn n115734(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 114520);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 126320);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 133398);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 137617);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 140159);
									s.is_empty()
								}
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
			super::push(out, 126321);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 133399);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 137618);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 140160);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 141691);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 142579);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 126322);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 133400);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 137619);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 126323);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 133401);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 137620);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 140161);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 141692);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 142580);
													match s {
														[] => true,
														['о', s @ ..] => super::t54::n144512(s, out),
														_ => false,
													}
												}
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
							super::push(out, 137621);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 140162);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рователь.
pub(crate) fn n115735(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 114521);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 126324);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 133402);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 133403);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 137622);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 126325);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 133404);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 137623);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 140163);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 141693);
											s.is_empty()
										}
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
					super::push(out, 133405);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 137624);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 140164);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 141694);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сователь.
pub(crate) fn n115736(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 114522);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 126326);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 133406);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 137625);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 140165);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тователь.
pub(crate) fn n115737(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 114523);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 126327);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 133407);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 137626);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 140166);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 137627);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 126328);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 133408);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 137628);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 140167);
									s.is_empty()
								}
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
			super::push(out, 126329);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 133409);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 137629);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 140168);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 137630);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 140169);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 141695);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 142581);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 126330);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 133410);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 137631);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 140170);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хователь.
pub(crate) fn n115738(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 114524);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 126331);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 133411);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 137632);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 140171);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рвовскрыватель.
pub(crate) fn n144035(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142582);
	s.is_empty()
}

/// Words ending -подбрасыватель.
pub(crate) fn n144036(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142583);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 143060);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -птыватель.
pub(crate) fn n127651(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126351);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 133432);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 137649);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 140182);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 141700);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -олог-изыскатель.
pub(crate) fn n144516(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143063);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 143303);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -летчик-испытатель.
pub(crate) fn n144873(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143420);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 143470);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нженер-испытатель.
pub(crate) fn n144874(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143421);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 143471);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -дитель-испытатель.
pub(crate) fn n144875(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143422);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 143472);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -авод-изготовитель.
pub(crate) fn n144768(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143315);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 143424);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ятие-изготовитель.
pub(crate) fn n144769(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143316);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 143425);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 143474);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ханик-водитель.
pub(crate) fn n144064(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142611);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 143080);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending --производитель.
pub(crate) fn n139151(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137729);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 140239);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 141739);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 142612);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 143081);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 143318);
											s.is_empty()
										}
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
					super::push(out, 141740);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 142613);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 143082);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 143319);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 143426);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 140240);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 141741);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 142614);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 143083);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 143320);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 143427);
													match s {
														[] => true,
														['п', s @ ..] => super::t54::n144928(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 140241);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 141742);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 142615);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 140242);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 141743);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 142616);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 143084);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 143321);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 143428);
													match s {
														[] => true,
														['ж', s @ ..] => super::t54::n144929(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 140243);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 141744);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 142617);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 143085);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 143322);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 143429);
													match s {
														[] => true,
														['о', s @ ..] => super::t54::n144930(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -автопроизводитель.
pub(crate) fn n144071(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142618);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 143086);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -моводитель.
pub(crate) fn n134908(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133532);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 137734);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 140248);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 141746);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 142619);
									s.is_empty()
								}
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
			super::push(out, 137735);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыводитель.
pub(crate) fn n127777(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126473);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 133533);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 137736);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 140249);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 141747);
									match s {
										[] => true,
										['я', s @ ..] => {
											super::push(out, 142620);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 143087);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сходитель.
pub(crate) fn n127778(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126474);
	s.is_empty()
}

/// The word приемник-распределитель.
pub(crate) fn n144954(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143501);
	s.is_empty()
}

/// Words ending -ка-распылитель.
pub(crate) fn n144080(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142627);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 143093);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 143326);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 143433);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 143479);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ол-заполнитель.
pub(crate) fn n144083(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142630);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 143095);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 143327);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 143434);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 143480);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ат-наполнитель.
pub(crate) fn n144084(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142631);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 143096);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 143328);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 143435);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 143481);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ав-исполнитель.
pub(crate) fn n144085(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142632);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 143097);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 143329);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 143436);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 143482);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											super::push(out, 143498);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ор-исполнитель.
pub(crate) fn n144086(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142633);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 143098);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 143330);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 143437);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ракета-носитель.
pub(crate) fn n144557(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143104);
	s.is_empty()
}

/// Words ending -ь-теплоноситель.
pub(crate) fn n144558(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143105);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 143334);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 143439);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 143484);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -казливость.
pub(crate) fn n135156(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133777);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 137891);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 140331);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 141800);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мазливость.
pub(crate) fn n135157(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133778);
	s.is_empty()
}

/// Words ending -езливость.
pub(crate) fn n128137(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126818);
	s.is_empty()
}

/// Words ending -озливость.
pub(crate) fn n128138(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126819);
	s.is_empty()
}

/// Words ending -узливость.
pub(crate) fn n128139(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 133779);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 133780);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 137892);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 140332);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 141801);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -язливость.
pub(crate) fn n128140(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126820);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 133781);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 137893);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йливость.
pub(crate) fn n116399(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115169);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 126821);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 133782);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 126822);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кливость.
pub(crate) fn n116400(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115170);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 126823);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лливость.
pub(crate) fn n116401(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115171);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 126824);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 133783);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мливость.
pub(crate) fn n116402(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115172);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 126825);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нливость.
pub(crate) fn n116403(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115173);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 126826);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 133784);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 126827);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 133785);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 126828);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 133786);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 137894);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пливость.
pub(crate) fn n116404(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115174);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 126829);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 133787);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 137895);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рливость.
pub(crate) fn n116405(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115175);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 126830);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 133788);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 137896);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 126831);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 133789);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 137897);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 140333);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 133790);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 133791);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 133792);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 137898);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 140334);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 141802);
											s.is_empty()
										}
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
					super::push(out, 133793);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 137899);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 140335);
									s.is_empty()
								}
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
			super::push(out, 126832);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 133794);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 126833);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 133795);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сливость.
pub(crate) fn n116406(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115176);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 126834);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 133796);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 126835);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 133797);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 137900);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 126836);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 133798);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 137901);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 133799);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 137902);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тливость.
pub(crate) fn n116407(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115177);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 126837);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 133800);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 137903);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 140336);
									s.is_empty()
								}
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
			super::push(out, 126838);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 133801);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 137904);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 140337);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', 'с', s @ ..] => {
					super::push(out, 133802);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 137905);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 133803);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 137906);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 140338);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 141803);
											s.is_empty()
										}
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
					super::push(out, 133804);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 137907);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 126839);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 133805);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 137908);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 126840);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 133806);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 137909);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 140339);
									s.is_empty()
								}
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
			super::push(out, 126841);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 133807);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 137910);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 140340);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 133808);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 137911);
							match s {
								[] => true,
								['щ', s @ ..] => {
									super::push(out, 140341);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 133809);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 137912);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 133810);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 137913);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 140342);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 141804);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 137914);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 140343);
									s.is_empty()
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
						['и', s @ ..] => {
							super::push(out, 137915);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 140344);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 141805);
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
								['о', s @ ..] => {
									super::push(out, 140345);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 141806);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 142655);
													match s {
														[] => true,
														['л', s @ ..] => super::t54::n144564(s, out),
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
									super::push(out, 140346);
									s.is_empty()
								}
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
			super::push(out, 126842);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 133811);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 126843);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 133812);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 137916);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 140347);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 140348);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 140349);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 141807);
											s.is_empty()
										}
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
					super::push(out, 133813);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 137917);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 140350);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 141808);
											s.is_empty()
										}
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
					super::push(out, 133814);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 137918);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 140351);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 141809);
											s.is_empty()
										}
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
					super::push(out, 133815);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 137919);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 140352);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 141810);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 126844);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 133816);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 126845);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 133817);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 126846);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 133818);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 137920);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 140353);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 141811);
											s.is_empty()
										}
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
					super::push(out, 133819);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 137921);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 140354);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чливость.
pub(crate) fn n116408(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115178);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 133820);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 137922);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 133821);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 137923);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 140355);
									s.is_empty()
								}
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
			super::push(out, 126847);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 133822);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 137924);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 126848);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 133823);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 137925);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 126849);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 133824);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 137926);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 126850);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шливость.
pub(crate) fn n116409(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115179);
	match s {
		[] => true,
		['е', 'м', 'с', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 133825);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 133826);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 126851);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 126852);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -бованность.
pub(crate) fn n135355(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133970);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 138021);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 140417);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 141841);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дованность.
pub(crate) fn n135356(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 138022);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 140418);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 138023);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 140419);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 138024);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 140420);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 141842);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 142667);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зованность.
pub(crate) fn n135357(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133971);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 138025);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 140421);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 138026);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 140422);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кованность.
pub(crate) fn n135358(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133972);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 138027);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 138028);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лованность.
pub(crate) fn n135359(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133973);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 138029);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 140423);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 141843);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 142668);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нованность.
pub(crate) fn n135360(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133974);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 138030);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 138031);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 140424);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 141844);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пованность.
pub(crate) fn n135361(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133975);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 138032);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 140425);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 141845);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 142669);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рованность.
pub(crate) fn n135362(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133976);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 138033);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 140426);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 141846);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 142670);
									s.is_empty()
								}
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
			super::push(out, 138034);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 140427);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 141847);
							match s {
								[] => true,
								['л', 'и', s @ ..] => {
									super::push(out, 142671);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 143115);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 143340);
													match s {
														[] => true,
														['р', s @ ..] => super::t54::n144894(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 140428);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 141848);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 142672);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 143116);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 143341);
													match s {
														[] => true,
														['г', s @ ..] => super::t54::n144895(s, out),
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
									super::push(out, 142673);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 140429);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 140430);
					match s {
						[] => true,
						['р', 'о', 'ф', s @ ..] => {
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 142674);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 142675);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 143117);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 143118);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 143342);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t54::n144896(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 140431);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 141849);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 142676);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 143119);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 143343);
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 143120);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 141850);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 140432);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 140433);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 140434);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 141851);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 140435);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 138035);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 140436);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 141852);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 142677);
									match s {
										[] => true,
										['ы', s @ ..] => {
											super::push(out, 143121);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 143344);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 138036);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 140437);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 141853);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 142678);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 142679);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сованность.
pub(crate) fn n135363(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133977);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 138037);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 140438);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 138038);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 140439);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 138039);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 140440);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тованность.
pub(crate) fn n135364(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133978);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 138040);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 140441);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 141854);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 138041);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 140442);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 141855);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фованность.
pub(crate) fn n135365(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133979);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 138042);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 140443);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 141856);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 142680);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рванность.
pub(crate) fn n128357(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127031);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 133980);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ганность.
pub(crate) fn n116620(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115386);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 127032);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -данность.
pub(crate) fn n116621(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115387);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 127033);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 133981);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 133982);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 138043);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 140444);
									match s {
										[] => true,
										['ы', s @ ..] => {
											super::push(out, 141857);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 142681);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 127034);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 133983);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 127035);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 127036);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 133984);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 138044);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 140445);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 141858);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 142682);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 133985);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 138045);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 127037);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -жанность.
pub(crate) fn n116622(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115388);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 127038);
			match s {
				[] => true,
				['е', 'д', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 138046);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 140446);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 138047);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 138048);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 140447);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 141859);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 141860);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 142683);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -занность.
pub(crate) fn n116623(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115389);
	match s {
		[] => true,
		['а', 'к', 'с', s @ ..] => {
			super::push(out, 127039);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 133986);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 138049);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 140448);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 133987);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 138050);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 140449);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 133988);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 138051);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 140450);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 141861);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 127040);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 133989);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -канность.
pub(crate) fn n116624(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115390);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 127041);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 133990);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 138052);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 127042);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ланность.
pub(crate) fn n116625(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115391);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 127043);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 133991);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 138053);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 140451);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 141862);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 127044);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 133992);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 133993);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 138054);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 127045);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 133994);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 138055);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 140452);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -манность.
pub(crate) fn n116626(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115392);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 127046);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 133995);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 127047);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 133996);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 138056);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 127048);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 133997);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 138057);
							s.is_empty()
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
					super::push(out, 133998);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 138058);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 138059);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 133999);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 138060);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 140453);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 138061);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 140454);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 138062);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 140455);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 138063);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 140456);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 141863);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 142684);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 134000);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 138064);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 140457);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нанность.
pub(crate) fn n116627(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115393);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 127049);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -панность.
pub(crate) fn n116628(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115394);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 127050);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 134001);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 127051);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ранность.
pub(crate) fn n116629(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115395);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 127052);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 127053);
			match s {
				[] => true,
				['з', s @ ..] => {
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 138065);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 138066);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 140458);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 141864);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 142685);
													match s {
														[] => true,
														['б', s @ ..] => super::t54::n144575(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 134002);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 138067);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 127054);
			match s {
				[] => true,
				['и', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 138068);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 140459);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 138069);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 140460);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 141865);
											s.is_empty()
										}
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
					super::push(out, 134003);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 138070);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 140461);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 138071);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 140462);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 134004);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 138072);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 127055);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 127056);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 134005);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 127057);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 134006);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 138073);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -санность.
pub(crate) fn n116630(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115396);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 127058);
			s.is_empty()
		}
		['и', 'п', 'ы', s @ ..] => {
			super::push(out, 127059);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 134007);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 138074);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -танность.
pub(crate) fn n116631(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115397);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 127060);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 127061);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 127062);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 134008);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 127063);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 134009);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 138075);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 140463);
									s.is_empty()
								}
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
			super::push(out, 127064);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 134010);
					match s {
						[] => true,
						['а', 'р', 'ы', s @ ..] => {
							super::push(out, 138076);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 140464);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 141866);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 127065);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 134011);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 138077);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 140465);
									s.is_empty()
								}
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
			super::push(out, 127066);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 134012);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 138078);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 140466);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 134013);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 138079);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 140467);
									s.is_empty()
								}
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
			super::push(out, 127067);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ханность.
pub(crate) fn n116632(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115398);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 127068);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 134014);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 138080);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 140468);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 141867);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 142686);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 127069);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 134015);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 138081);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 140469);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 141868);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шанность.
pub(crate) fn n116633(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115399);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 127070);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 134016);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -амозабвенность.
pub(crate) fn n144140(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142687);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 143123);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -многозвенность.
pub(crate) fn n144141(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142688);
	s.is_empty()
}

/// Words ending -твенность.
pub(crate) fn n128404(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127077);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 134025);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 138091);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 138092);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 140474);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 141872);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 138093);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 140475);
							match s {
								[] => true,
								['р', 'н', 'з', s @ ..] => {
									super::push(out, 141873);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 142689);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 140476);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 138094);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 140477);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 141874);
									match s {
										[] => true,
										['с', 'о', 'п', 'е', s @ ..] => {
											super::push(out, 142690);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 140478);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 138095);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 140479);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 141875);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ж', s @ ..] => {
							super::push(out, 140480);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 141876);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 142691);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 143124);
													match s {
														[] => true,
														['н', s @ ..] => super::t54::n144798(s, out),
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
									super::push(out, 141877);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 142692);
											s.is_empty()
										}
										['д', s @ ..] => {
											super::push(out, 142693);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 143125);
													match s {
														[] => true,
														['х', s @ ..] => super::t54::n144799(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 142694);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 143126);
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 142695);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 143127);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => super::t54::n143327(s, out),
								['у', s @ ..] => super::t54::n143328(s, out),
								_ => false,
							}
						}
						['т', s @ ..] => super::t54::n141926(s, out),
						['ч', s @ ..] => super::t54::n141927(s, out),
						['щ', s @ ..] => super::t54::n141928(s, out),
						_ => false,
					}
				}
				['й', s @ ..] => super::t54::n139525(s, out),
				['м', s @ ..] => super::t54::n139526(s, out),
				['н', s @ ..] => super::t54::n139527(s, out),
				['р', s @ ..] => super::t54::n139528(s, out),
				['с', s @ ..] => super::t54::n139529(s, out),
				['т', s @ ..] => super::t54::n139530(s, out),
				['ь', s @ ..] => super::t54::n139531(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -денность.
pub(crate) fn n116638(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115403);
	match s {
		[] => true,
		['ж', s @ ..] => {
			super::push(out, 127080);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 134027);
					s.is_empty()
				}
				['о', 'р', 'ы', s @ ..] => {
					super::push(out, 134028);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 138104);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 140496);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 141893);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 142712);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', 'н', s @ ..] => {
					super::push(out, 134029);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 138105);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 138106);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 140497);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 141894);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 127081);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 134030);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 138107);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 140498);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 141895);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 142713);
													match s {
														[] => true,
														['е', s @ ..] => super::t54::n144592(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 127082);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 134031);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 138108);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -женность.
pub(crate) fn n116639(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115404);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 127083);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 134032);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 138109);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 140499);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 134033);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 138110);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 134034);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 138111);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 140500);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 134035);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 138112);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 140501);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 138113);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 140502);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 141896);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 142714);
													s.is_empty()
												}
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
							super::push(out, 138114);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 140503);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 138115);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 140504);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 141897);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 142715);
													s.is_empty()
												}
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
							super::push(out, 138116);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 140505);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 141898);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', 'н', s @ ..] => {
			super::push(out, 127084);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 127085);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 134036);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 138117);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 140506);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 138118);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 140507);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 134037);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 127086);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 127087);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 134038);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 138119);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 127088);
			match s {
				[] => true,
				['л', 'о', s @ ..] => {
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 138120);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 140508);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 138121);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 140509);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 141899);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 142716);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 134039);
					match s {
						[] => true,
						['о', 'в', s @ ..] => {
							super::push(out, 138122);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 140510);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 141900);
											s.is_empty()
										}
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
					super::push(out, 134040);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 127089);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 134041);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 138123);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 127090);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 134042);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 134043);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 138124);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 140511);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 134044);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 138125);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 140512);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 140513);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 141901);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 138126);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 140514);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 141902);
											s.is_empty()
										}
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
					super::push(out, 134045);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 127091);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 134046);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 138127);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 140515);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 141903);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 140516);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 141904);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 142717);
													s.is_empty()
												}
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
							super::push(out, 138128);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 140517);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 141905);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => super::t54::n135438(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зенность.
pub(crate) fn n116640(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115405);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 127092);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 134048);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ленность.
pub(crate) fn n116641(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115406);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 127093);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 134049);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 134050);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 127094);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 134051);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 138130);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 140519);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 134052);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 134053);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 138131);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 140520);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 134054);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 138132);
							match s {
								[] => true,
								['щ', s @ ..] => {
									super::push(out, 140521);
									match s {
										[] => true,
										['ы', s @ ..] => {
											super::push(out, 141906);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 142718);
													s.is_empty()
												}
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
								['г', s @ ..] => {
									super::push(out, 140522);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 141907);
											s.is_empty()
										}
										_ => false,
									}
								}
								['к', s @ ..] => {
									super::push(out, 140523);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 141908);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 142719);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 134055);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 138133);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 138134);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 140524);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 141909);
											s.is_empty()
										}
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
					super::push(out, 134056);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 138135);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 140525);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 141910);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 140526);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 141911);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 142720);
													match s {
														[] => true,
														['н', s @ ..] => super::t54::n144593(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 127095);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 134057);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 138136);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 138137);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 140527);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 138138);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 134058);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 138139);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 140528);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 141912);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 142721);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 134059);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 138140);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 140529);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 134060);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 138141);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 140530);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ж', s @ ..] => {
							super::push(out, 138142);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 140531);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 138143);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 140532);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 141913);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 138144);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 140533);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 141914);
											s.is_empty()
										}
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
						['к', s @ ..] => super::t54::n139575(s, out),
						['л', s @ ..] => {
							super::push(out, 138146);
							match s {
								[] => true,
								['с', s @ ..] => super::t54::n141980(s, out),
								_ => false,
							}
						}
						['н', s @ ..] => super::t54::n139577(s, out),
						['т', s @ ..] => super::t54::n139578(s, out),
						_ => false,
					}
				}
				['т', s @ ..] => super::t54::n135453(s, out),
				_ => false,
			}
		}
		['г', s @ ..] => super::t54::n128423(s, out),
		['д', s @ ..] => super::t54::n128424(s, out),
		['е', s @ ..] => super::t54::n128425(s, out),
		['и', s @ ..] => super::t54::n128426(s, out),
		['к', s @ ..] => super::t54::n128427(s, out),
		['м', s @ ..] => super::t54::n128428(s, out),
		['о', s @ ..] => super::t54::n128429(s, out),
		['п', s @ ..] => super::t54::n128430(s, out),
		['с', s @ ..] => super::t54::n128431(s, out),
		['т', s @ ..] => super::t54::n128432(s, out),
		['ш', s @ ..] => super::t54::n128433(s, out),
		['ы', s @ ..] => super::t54::n128434(s, out),
		_ => false,
	}
}

/// Words ending -менность.
pub(crate) fn n116642(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115407);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 127107);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 127108);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 134086);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 138177);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 127109);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 134087);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 138178);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 140561);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 134088);
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 140562);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 141934);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 140563);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 141935);
											s.is_empty()
										}
										['к', s @ ..] => {
											super::push(out, 141936);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 142728);
													match s {
														[] => true,
														['а', s @ ..] => super::t54::n144596(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 142729);
													match s {
														[] => true,
														['о', s @ ..] => super::t54::n144597(s, out),
														_ => false,
													}
												}
												['з', s @ ..] => {
													super::push(out, 142730);
													match s {
														[] => true,
														['а', s @ ..] => super::t54::n144598(s, out),
														_ => false,
													}
												}
												['и', s @ ..] => {
													super::push(out, 142731);
													match s {
														[] => true,
														['д', s @ ..] => super::t54::n144599(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 141937);
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
								['б', s @ ..] => {
									super::push(out, 140564);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 140565);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 141938);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 127110);
			match s {
				[] => true,
				['и', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 138179);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 140566);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 138180);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 127111);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 134089);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 127112);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 134090);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 138181);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 127113);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 134091);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 134092);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 127114);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 134093);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 127115);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 134094);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 138182);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 140567);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 141939);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 127116);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 134095);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 138183);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 140568);
									s.is_empty()
								}
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
			super::push(out, 127117);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 134096);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 138184);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 140569);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ненность.
pub(crate) fn n116643(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115408);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 127118);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 134097);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 138185);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 140570);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 141940);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 142732);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 134098);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 138186);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 140571);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 141941);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 142733);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 134099);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 138187);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 140572);
									s.is_empty()
								}
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
			super::push(out, 127119);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 134100);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 138188);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 140573);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 141942);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 127120);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 134101);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 127121);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 134102);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 134103);
					s.is_empty()
				}
				['ц', 'с', s @ ..] => {
					super::push(out, 134104);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 138189);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 140574);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 141943);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 142734);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 127122);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 134105);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 127123);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 134106);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 138190);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 134107);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 138191);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 134108);
					s.is_empty()
				}
				['я', s @ ..] => {
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 138192);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 138193);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 140575);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 138194);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 140576);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 141944);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 142735);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 127124);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 134109);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 134110);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 127125);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 134111);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 138195);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 140577);
									s.is_empty()
								}
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
			super::push(out, 127126);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 127127);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 134112);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 138196);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 134113);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 138197);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 138198);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 140578);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 141945);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 142736);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t54::n144600(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 127128);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 134114);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 138199);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оенность.
pub(crate) fn n116644(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115409);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 127129);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 127130);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 134115);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 127131);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 134116);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 138200);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 140579);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 141946);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 127132);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 134117);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 127133);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 134118);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 140580);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 140581);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 141947);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 142737);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пенность.
pub(crate) fn n116645(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115410);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 127134);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 134119);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 138201);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ренность.
pub(crate) fn n116646(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115411);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 134120);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 134121);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 138202);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 134122);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 138203);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 134123);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 138204);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 134124);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 138205);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', 'у', s @ ..] => {
			super::push(out, 127135);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 134125);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 127136);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 134126);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 138206);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 140582);
									s.is_empty()
								}
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
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 134127);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 138207);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', 'м', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 138208);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 140583);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 138209);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 127137);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 134128);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 138210);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 140584);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 141948);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 138211);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 140585);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 141949);
											s.is_empty()
										}
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
					super::push(out, 134129);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 138212);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 134130);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 138213);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 140586);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 141950);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 127138);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 134131);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 138214);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 140587);
									s.is_empty()
								}
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
			super::push(out, 127139);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 134132);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 138215);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 127140);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 134133);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 138216);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 140588);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 141951);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 127141);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 134134);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 138217);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 138218);
							match s {
								[] => true,
								['е', 'л', 'в', 'о', 'д', 'у', 'е', s @ ..] => {
									super::push(out, 140589);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 141952);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 140590);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 134135);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 138219);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 140591);
									s.is_empty()
								}
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
			super::push(out, 127142);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 134136);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 127143);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 134137);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 138220);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 140592);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 141953);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 142738);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 134138);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 138221);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 140593);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 134139);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 138222);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 140594);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 138223);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 140595);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 141954);
											s.is_empty()
										}
										_ => false,
									}
								}
								['б', s @ ..] => {
									super::push(out, 140596);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 141955);
											s.is_empty()
										}
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
					super::push(out, 134140);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 138224);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 140597);
									s.is_empty()
								}
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
				['м', s @ ..] => {
					super::push(out, 134141);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 138225);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 134142);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 138226);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 140598);
									s.is_empty()
								}
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
			super::push(out, 127144);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 134143);
					s.is_empty()
				}
				['о', s @ ..] => super::t54::n135542(s, out),
				_ => false,
			}
		}
		['ы', s @ ..] => super::t54::n128476(s, out),
		['я', s @ ..] => super::t54::n128477(s, out),
		_ => false,
	}
}

/// Words ending -сенность.
pub(crate) fn n116647(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115412);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 127147);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 134147);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 134148);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 134149);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 138229);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -етенность.
pub(crate) fn n128479(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127148);
	s.is_empty()
}

/// Words ending -уенность.
pub(crate) fn n116649(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115414);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 127149);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 134150);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ценность.
pub(crate) fn n116650(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115415);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 127150);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 134151);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 134152);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 138230);
							match s {
								[] => true,
								['о', 'п', 'е', s @ ..] => {
									super::push(out, 140601);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ченность.
pub(crate) fn n116651(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115416);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 127151);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 134153);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 134154);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 134155);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 138231);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 140602);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 138232);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 140603);
									s.is_empty()
								}
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
			super::push(out, 127152);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 134156);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 138233);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 140604);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 134157);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 138234);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 127153);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 134158);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 138235);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 134159);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 138236);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 140605);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 134160);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 138237);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 134161);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 138238);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 140606);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 141956);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 138239);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 140607);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 141957);
											s.is_empty()
										}
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
					super::push(out, 134162);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 138240);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 140608);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 138241);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 140609);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 141958);
											s.is_empty()
										}
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
					super::push(out, 134163);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 138242);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 140610);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 141959);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 127154);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 134164);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 127155);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 134165);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 138243);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 138244);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 140611);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 138245);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 140612);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 141960);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 142739);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 127156);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 134166);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 138246);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 140613);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 138247);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 134167);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 138248);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 140614);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 141961);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 142740);
													match s {
														[] => true,
														['у', s @ ..] => super::t54::n144601(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 134168);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 138249);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 140615);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 138250);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 140616);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 134169);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 138251);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 134170);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 138252);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 140617);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 141962);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 138253);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 140618);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 141963);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 142741);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 127157);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 134171);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 138254);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => super::t54::n135571(s, out),
				_ => false,
			}
		}
		['у', s @ ..] => super::t54::n128489(s, out),
		['ю', s @ ..] => super::t54::n128490(s, out),
		['я', s @ ..] => super::t54::n128491(s, out),
		_ => false,
	}
}

/// Words ending -шенность.
pub(crate) fn n116652(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115417);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 127161);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 134179);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 127162);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 134180);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 138267);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 138268);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 140631);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 141970);
											s.is_empty()
										}
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
					super::push(out, 134181);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 138269);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 140632);
									s.is_empty()
								}
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
			super::push(out, 127163);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 134182);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 134183);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 134184);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 138270);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 134185);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 138271);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 140633);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 141971);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 142743);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 127164);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 134186);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 138272);
							s.is_empty()
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
				['д', s @ ..] => {
					super::push(out, 134187);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 138273);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 140634);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 134188);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 138274);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 140635);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 134189);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 138275);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 134190);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 138276);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 138277);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 134191);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 138278);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 138279);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 134192);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 138280);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 140636);
									s.is_empty()
								}
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
			super::push(out, 127165);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 134193);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 138281);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щенность.
pub(crate) fn n116653(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115418);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 127166);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 134194);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 127167);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 134195);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 127168);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 134196);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 138282);
							match s {
								[] => true,
								['о', 'р', s @ ..] => {
									super::push(out, 140637);
									s.is_empty()
								}
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
			super::push(out, 127169);
			match s {
				[] => true,
				['щ', s @ ..] => {
					super::push(out, 134197);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 138283);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 127170);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 127171);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 134198);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 138284);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 134199);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 134200);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 138285);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 140638);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 138286);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 140639);
									s.is_empty()
								}
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
			super::push(out, 127172);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 134201);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 134202);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 138287);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 140640);
									s.is_empty()
								}
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
			super::push(out, 127173);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 134203);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 138288);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 140641);
									s.is_empty()
								}
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
			super::push(out, 127174);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 134204);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 138289);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -процессорность.
pub(crate) fn n144197(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142744);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 143149);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -огофакторность.
pub(crate) fn n144198(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142745);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 143150);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ноговекторность.
pub(crate) fn n144199(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142746);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 143151);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -безошибочность.
pub(crate) fn n144206(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142753);
	s.is_empty()
}

/// Words ending -зоговорочность.
pub(crate) fn n144207(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142754);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 143156);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 143356);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -осочность.
pub(crate) fn n128672(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 134346);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 138389);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 140696);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 134347);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -точность.
pub(crate) fn n116795(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115551);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 127334);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 134348);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 138390);
							match s {
								[] => true,
								['о', 'д', 'е', s @ ..] => {
									super::push(out, 140697);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 142000);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 127335);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 134349);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 138391);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 127336);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 134350);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 138392);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 138393);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 140698);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 134351);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 127337);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 134352);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 138394);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 140699);
									s.is_empty()
								}
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
			super::push(out, 127338);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 127339);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 134353);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -корострельность.
pub(crate) fn n144611(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143158);
	s.is_empty()
}

/// Words ending -вательность.
pub(crate) fn n139851(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138416);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 140713);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 142009);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 142759);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 143159);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 143357);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 143448);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 142010);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 142760);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 143160);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 143358);
											s.is_empty()
										}
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
					super::push(out, 142011);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 142761);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 143161);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 143359);
											s.is_empty()
										}
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
					super::push(out, 142012);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 142762);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 143162);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 143360);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 143449);
													s.is_empty()
												}
												_ => false,
											}
										}
										['з', s @ ..] => {
											super::push(out, 143361);
											s.is_empty()
										}
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
					super::push(out, 142013);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 142763);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 143163);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 143362);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -трогательность.
pub(crate) fn n144217(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142764);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 143164);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -дательность.
pub(crate) fn n139853(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138418);
	s.is_empty()
}

/// Words ending -жательность.
pub(crate) fn n139854(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138419);
	s.is_empty()
}

/// Words ending -зательность.
pub(crate) fn n139855(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138420);
	s.is_empty()
}

/// Words ending -кательность.
pub(crate) fn n139856(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138421);
	s.is_empty()
}

/// Words ending -лательность.
pub(crate) fn n139857(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138422);
	s.is_empty()
}

/// Words ending -мательность.
pub(crate) fn n139858(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138423);
	s.is_empty()
}

/// Words ending -нательность.
pub(crate) fn n139859(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138424);
	match s {
		[] => true,
		['з', 'о', s @ ..] => {
			super::push(out, 140716);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 142015);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 142016);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рательность.
pub(crate) fn n139860(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138425);
	s.is_empty()
}

/// Words ending -тательность.
pub(crate) fn n139861(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138426);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 140717);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 142017);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цательность.
pub(crate) fn n139862(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138427);
	s.is_empty()
}

/// Words ending -чательность.
pub(crate) fn n139863(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138428);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 140718);
			match s {
				[] => true,
				['м', 'и', 'р', 'п', 'о', s @ ..] => {
					super::push(out, 142018);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 142765);
							match s {
								[] => true,
								['с', 'о', 'д', '\u{1}', s @ ..] => {
									super::push(out, 143165);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щательность.
pub(crate) fn n139864(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138429);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 140719);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ительность.
pub(crate) fn n135787(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134385);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 138430);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 138431);
			s.is_empty()
		}
		['з', 'а', s @ ..] => {
			super::push(out, 138432);
			match s {
				[] => true,
				['р', 'б', s @ ..] => {
					super::push(out, 140720);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 138433);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 138434);
			s.is_empty()
		}
		['р', 'з', s @ ..] => {
			super::push(out, 138435);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 140721);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 142019);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 138436);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 140722);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 142020);
							s.is_empty()
						}
						['е', 'м', 'в', s @ ..] => {
							super::push(out, 142021);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', 'а', s @ ..] => {
			super::push(out, 138437);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 138438);
			match s {
				[] => true,
				['е', 'р', s @ ..] => {
					super::push(out, 140723);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -отельность.
pub(crate) fn n135788(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134386);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 138439);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 140724);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 142022);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 142766);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 143166);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ртельность.
pub(crate) fn n135789(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134387);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 138440);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 140725);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 142023);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 142767);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -стельность.
pub(crate) fn n135790(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134388);
	s.is_empty()
}

/// Words ending -ятельность.
pub(crate) fn n135791(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134389);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 138441);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 140726);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 138442);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 140727);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 142024);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цельность.
pub(crate) fn n128704(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127365);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 134390);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 138443);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 140728);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 134391);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 138444);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 140729);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 134392);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 138445);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 140730);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ноапостольность.
pub(crate) fn n144620(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143167);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 143363);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рогаммаспектрометрия.
pub(crate) fn n144817(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143364);
	s.is_empty()
}

/// Words ending -реплавательница.
pub(crate) fn n144290(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142837);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 143188);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хоплавательница.
pub(crate) fn n144291(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142838);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 143189);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -жертвовательница.
pub(crate) fn n144292(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142839);
	s.is_empty()
}

/// Words ending -следовательница.
pub(crate) fn n144293(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142840);
	s.is_empty()
}

/// Words ending -бразовательница.
pub(crate) fn n144294(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142841);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 143190);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьзовательница.
pub(crate) fn n142467(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141022);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 142185);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 142842);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 143191);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ковательница.
pub(crate) fn n140472(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139035);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 141023);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 142186);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 142843);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -новательница.
pub(crate) fn n140473(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139036);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 141024);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 142187);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ровательница.
pub(crate) fn n140474(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139037);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 141025);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ывательница.
pub(crate) fn n137011(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135605);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 139038);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 141026);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 142188);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 142844);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 143192);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 139039);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 141027);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 142189);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 142845);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 143193);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 143370);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 139040);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 141028);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дательница.
pub(crate) fn n131071(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129719);
	s.is_empty()
}

/// Words ending -жательница.
pub(crate) fn n131072(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129720);
	s.is_empty()
}

/// Words ending -кательница.
pub(crate) fn n131073(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129721);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 135606);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 139041);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 139042);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 141029);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 142190);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 142846);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лательница.
pub(crate) fn n131074(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129722);
	match s {
		[] => true,
		['е', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 139043);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 141030);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 139044);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рательница.
pub(crate) fn n131075(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129723);
	s.is_empty()
}

/// Words ending -тательница.
pub(crate) fn n131076(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129724);
	s.is_empty()
}

/// Words ending -шательница.
pub(crate) fn n131077(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129725);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 135607);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 139045);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 135608);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 139046);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 135609);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 139047);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 141031);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 142191);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щательница.
pub(crate) fn n131078(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129726);
	s.is_empty()
}

/// Words ending -ительница.
pub(crate) fn n121036(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119773);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 129727);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 129728);
			match s {
				[] => true,
				['о', 'р', s @ ..] => {
					super::push(out, 135610);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 139048);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 141032);
									match s {
										[] => true,
										['п', '-', s @ ..] => {
											super::push(out, 142192);
											match s {
												[] => true,
												['я', s @ ..] => {
													super::push(out, 142847);
													match s {
														[] => true,
														['н', s @ ..] => super::t54::n144647(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 129729);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 135611);
					match s {
						[] => true,
						['р', 'х', 'о', s @ ..] => {
							super::push(out, 139049);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 141033);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 142193);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 142848);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t54::n144648(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 129730);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 135612);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 139050);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 141034);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 142194);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 142849);
													match s {
														[] => true,
														['р', s @ ..] => super::t54::n144649(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 129731);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 129732);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 135613);
					match s {
						[] => true,
						['р', 'а', s @ ..] => {
							super::push(out, 139051);
							match s {
								[] => true,
								['н', 'о', s @ ..] => {
									super::push(out, 141035);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 142195);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 142850);
													match s {
														[] => true,
														['р', s @ ..] => super::t54::n144650(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -стельница.
pub(crate) fn n121037(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119774);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 129733);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 135614);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 139052);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ятельница.
pub(crate) fn n121038(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119775);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 129734);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 135615);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 139053);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 135616);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 139054);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 129735);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 135617);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word выгравирование.
pub(crate) fn n144319(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142866);
	s.is_empty()
}

/// Words ending -бомбардирование.
pub(crate) fn n144320(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142867);
	s.is_empty()
}

/// Words ending -переаранжирование.
pub(crate) fn n144321(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142868);
	s.is_empty()
}

/// Words ending -кирование.
pub(crate) fn n121689(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120422);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 130120);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 135838);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 139171);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 130121);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 130122);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 130123);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 130124);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 135839);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 139172);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 141105);
									s.is_empty()
								}
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
			super::push(out, 130125);
			match s {
				[] => true,
				['а', 'м', '\u{1}', s @ ..] => {
					super::push(out, 135840);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лирование.
pub(crate) fn n121690(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120423);
	match s {
		[] => true,
		['а', 'м', s @ ..] => {
			super::push(out, 130126);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 135841);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 139173);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 130127);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 135842);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 139174);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 141106);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', 'к', s @ ..] => {
			super::push(out, 130128);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 135843);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 139175);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 141107);
									s.is_empty()
								}
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
			super::push(out, 130129);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 130130);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 135844);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', 'п', s @ ..] => {
			super::push(out, 130131);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 135845);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 130132);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мирование.
pub(crate) fn n121691(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120424);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 130133);
			s.is_empty()
		}
		['и', 'р', s @ ..] => {
			super::push(out, 130134);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 135846);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 139176);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 130135);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 135847);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 130136);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 130137);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 135848);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 139177);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 141108);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 141109);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 142234);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 139178);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 141110);
									s.is_empty()
								}
								['а', 'р', 'р', 'е', 'т', '\u{1}', s @ ..] => {
									super::push(out, 141111);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 141112);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 142235);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 142869);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 141113);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 142236);
											s.is_empty()
										}
										['р', 'е', s @ ..] => {
											super::push(out, 142237);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 142870);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 141114);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 142238);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 141115);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 142239);
											s.is_empty()
										}
										['м', s @ ..] => {
											super::push(out, 142240);
											s.is_empty()
										}
										['р', s @ ..] => {
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 142871);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 142872);
													match s {
														[] => true,
														['л', s @ ..] => super::t54::n144658(s, out),
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
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 142241);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 142242);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 142873);
													s.is_empty()
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 142243);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 142874);
													match s {
														[] => true,
														['р', s @ ..] => super::t54::n144659(s, out),
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
									super::push(out, 141116);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 142244);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 142875);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нирование.
pub(crate) fn n121692(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120425);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 130138);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 135849);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 139179);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 130139);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 130140);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 135850);
					match s {
						[] => true,
						['ц', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 141117);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 142245);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 142876);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['к', s @ ..] => {
									super::push(out, 141118);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 135851);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 139180);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 135852);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 139181);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пирование.
pub(crate) fn n121693(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120426);
	match s {
		[] => true,
		['а', 'р', s @ ..] => {
			super::push(out, 130141);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 135853);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 139182);
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
				['к', s @ ..] => {
					super::push(out, 135854);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 139183);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 141119);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 135855);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 130142);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 130143);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 135856);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 139184);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 141120);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фрирование.
pub(crate) fn n131499(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130144);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 135857);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 139185);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 141121);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сирование.
pub(crate) fn n121695(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 130145);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 130146);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 130147);
			match s {
				[] => true,
				['а', 'п', s @ ..] => {
					super::push(out, 135858);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 139186);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 135859);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 139187);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 141122);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 141123);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 142246);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 141124);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тирование.
pub(crate) fn n121696(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120428);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 130148);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 135860);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 135861);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 139188);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 130149);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 130150);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 130151);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 130152);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 130153);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 130154);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 135862);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 139189);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 141125);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 142247);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 142877);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 135863);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 139190);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 139191);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 141126);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 141127);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 142248);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 142878);
													match s {
														[] => true,
														['е', s @ ..] => super::t54::n144660(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', 'ю', s @ ..] => {
			super::push(out, 130155);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 135864);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уирование.
pub(crate) fn n121697(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120429);
	s.is_empty()
}

/// Words ending -фирование.
pub(crate) fn n121698(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120430);
	s.is_empty()
}

/// Words ending -цирование.
pub(crate) fn n121699(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120431);
	match s {
		[] => true,
		['и', 'р', s @ ..] => {
			super::push(out, 130156);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 135865);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 139192);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 141128);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ширование.
pub(crate) fn n121700(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120432);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 130157);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 130158);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 135866);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 139193);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 141129);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 142249);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 130159);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 135867);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 139194);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 141130);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 139195);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 141131);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нтизападничество.
pub(crate) fn n144711(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143258);
	s.is_empty()
}

/// Words ending -ьяволопоклонничество.
pub(crate) fn n144712(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143259);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 143396);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -расстреливатель.
pub(crate) fn n144506(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143053);
	s.is_empty()
}

/// Words ending -тернет-пользователь.
pub(crate) fn n144946(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143493);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 143500);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -отец-основатель.
pub(crate) fn n144512(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143059);
	s.is_empty()
}

/// Words ending -приятие-производитель.
pub(crate) fn n144928(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143475);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 143495);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -жеребец-производитель.
pub(crate) fn n144929(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143476);
	s.is_empty()
}

/// Words ending -омпания-производитель.
pub(crate) fn n144930(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143477);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 143496);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ловоохотливость.
pub(crate) fn n144564(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143111);
	s.is_empty()
}

/// Words ending -ривилегированность.
pub(crate) fn n144894(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143441);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 143485);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -глинизированность.
pub(crate) fn n144895(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143442);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 143486);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 143499);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word несформированность.
pub(crate) fn n144896(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143443);
	s.is_empty()
}

/// Words ending -безвозбранность.
pub(crate) fn n144575(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143122);
	s.is_empty()
}

/// Words ending -невежественность.
pub(crate) fn n144798(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143345);
	s.is_empty()
}

/// Words ending -художественность.
pub(crate) fn n144799(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143346);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 143444);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 143487);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ржественность.
pub(crate) fn n143327(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141878);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 142696);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 143128);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ужественность.
pub(crate) fn n143328(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141879);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 142697);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 142698);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 143129);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тественность.
pub(crate) fn n141926(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140481);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 141880);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 142699);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 143130);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 143347);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чественность.
pub(crate) fn n141927(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140482);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 141881);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 142700);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 143131);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 141882);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 142701);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щественность.
pub(crate) fn n141928(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140483);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 141883);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 142702);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 141884);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 142703);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 142704);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 143132);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 143348);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 142705);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 143133);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 143349);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 143445);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 143488);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 142706);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йственность.
pub(crate) fn n139525(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138096);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 140484);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 140485);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 140486);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мственность.
pub(crate) fn n139526(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138097);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 140487);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 141885);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 142707);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 143134);
									s.is_empty()
								}
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
			super::push(out, 140488);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 141886);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 142708);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 143135);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 141887);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 142709);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 143136);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нственность.
pub(crate) fn n139527(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138098);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 140489);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 140490);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 141888);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 140491);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 141889);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рственность.
pub(crate) fn n139528(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138099);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 140492);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сственность.
pub(crate) fn n139529(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138100);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 140493);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 141890);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тственность.
pub(crate) fn n139530(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138101);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 140494);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 141891);
					match s {
						[] => true,
						['т', 'о', 'о', s @ ..] => {
							super::push(out, 142710);
							match s {
								[] => true,
								['с', 'е', s @ ..] => {
									super::push(out, 143137);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 143350);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 143446);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьственность.
pub(crate) fn n139531(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138102);
	match s {
		[] => true,
		['л', 'е', s @ ..] => {
			super::push(out, 140495);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 141892);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 142711);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 143138);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 143351);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 143447);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -епредвиденность.
pub(crate) fn n144592(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143139);
	s.is_empty()
}

/// Words ending -тяженность.
pub(crate) fn n135438(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134047);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 138129);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 140518);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -недолюбленность.
pub(crate) fn n144593(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143140);
	s.is_empty()
}

/// Words ending -ковленность.
pub(crate) fn n139575(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138145);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 140534);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 141915);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 142722);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 143141);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -словленность.
pub(crate) fn n141980(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140535);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 141916);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -новленность.
pub(crate) fn n139577(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138147);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 140536);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 141917);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 140537);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 141918);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 142723);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -товленность.
pub(crate) fn n139578(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138148);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 140538);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 141919);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -твленность.
pub(crate) fn n135453(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134061);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 138149);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 140539);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 141920);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 142724);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гленность.
pub(crate) fn n128423(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127096);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 134062);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 138150);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 140540);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дленность.
pub(crate) fn n128424(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127097);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 134063);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 138151);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еленность.
pub(crate) fn n128425(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127098);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 134064);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 134065);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 138152);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 134066);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 138153);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 140541);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 141921);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иленность.
pub(crate) fn n128426(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 134067);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 138154);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 140542);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 134068);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 138155);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 140543);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кленность.
pub(crate) fn n128427(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127099);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 134069);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 138156);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мленность.
pub(crate) fn n128428(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127100);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 134070);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 138157);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 138158);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 140544);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 141922);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 138159);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 140545);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 141923);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 142725);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 143142);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', 'д', s @ ..] => {
					super::push(out, 138160);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 140546);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 141924);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 142726);
											s.is_empty()
										}
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
					super::push(out, 138161);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 140547);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 134071);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 138162);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 140548);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 141925);
									s.is_empty()
								}
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
			super::push(out, 134072);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оленность.
pub(crate) fn n128429(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127101);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 134073);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 138163);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 140549);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 141926);
									s.is_empty()
								}
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
			super::push(out, 134074);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 138164);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 134075);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 134076);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 134077);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 138165);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 140550);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 141927);
									s.is_empty()
								}
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
			super::push(out, 134078);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 138166);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 140551);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 134079);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 138167);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 140552);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 141928);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пленность.
pub(crate) fn n128430(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127102);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 134080);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 138168);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 140553);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 141929);
									s.is_empty()
								}
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
			super::push(out, 134081);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 138169);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 140554);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 138170);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 140555);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 141930);
									s.is_empty()
								}
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
				['к', s @ ..] => {
					super::push(out, 138171);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 140556);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 141931);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 142727);
											s.is_empty()
										}
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
					super::push(out, 138172);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 140557);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 141932);
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
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							super::push(out, 140558);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 140559);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сленность.
pub(crate) fn n128431(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127103);
	match s {
		[] => true,
		['и', s @ ..] => {
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 138173);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 140560);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 141933);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 138174);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 134082);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 138175);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тленность.
pub(crate) fn n128432(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127104);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 134083);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шленность.
pub(crate) fn n128433(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127105);
	match s {
		[] => true,
		['ы', s @ ..] => {
			super::push(out, 134084);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 138176);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыленность.
pub(crate) fn n128434(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127106);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 134085);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -атковременность.
pub(crate) fn n144596(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143143);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 143352);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -одновременность.
pub(crate) fn n144597(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143144);
	s.is_empty()
}

/// Words ending -азновременность.
pub(crate) fn n144598(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143145);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 143353);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -диновременность.
pub(crate) fn n144599(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143146);
	s.is_empty()
}

/// The word невыясненность.
pub(crate) fn n144600(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143147);
	s.is_empty()
}

/// Words ending -ощренность.
pub(crate) fn n135542(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134144);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 138227);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 140599);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыренность.
pub(crate) fn n128476(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127145);
	match s {
		[] => true,
		['ф', s @ ..] => {
			super::push(out, 134145);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -яренность.
pub(crate) fn n128477(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127146);
	match s {
		[] => true,
		['ъ', s @ ..] => {
			super::push(out, 134146);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 138228);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 140600);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -упорядоченность.
pub(crate) fn n144601(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143148);
	s.is_empty()
}

/// Words ending -орченность.
pub(crate) fn n135571(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 138255);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 140619);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 138256);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ученность.
pub(crate) fn n128489(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127158);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 134172);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 138257);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 134173);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 138258);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 140620);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 138259);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 140621);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 141964);
									s.is_empty()
								}
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
			super::push(out, 134174);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 138260);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 140622);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 141965);
									s.is_empty()
								}
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
			super::push(out, 134175);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 138261);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 140623);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 138262);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 140624);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 141966);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 138263);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 140625);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юченность.
pub(crate) fn n128490(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127159);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 134176);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 138264);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 140626);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 140627);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 141967);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 140628);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 141968);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 142742);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 134177);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 138265);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 140629);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яченность.
pub(crate) fn n128491(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127160);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 134178);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 138266);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 140630);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 141969);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ня-прародительница.
pub(crate) fn n144647(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143194);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 143371);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 143451);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 143489);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word дарохранительница.
pub(crate) fn n144648(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143195);
	s.is_empty()
}

/// Words ending -ромакурительница.
pub(crate) fn n144649(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143196);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 143372);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -равонарушительница.
pub(crate) fn n144650(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143197);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 143373);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 143452);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лороформирование.
pub(crate) fn n144658(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143205);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 143375);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рансформирование.
pub(crate) fn n144659(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143206);
	s.is_empty()
}

/// Words ending -евмосортирование.
pub(crate) fn n144660(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143207);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 143376);
			s.is_empty()
		}
		_ => false,
	}
}

