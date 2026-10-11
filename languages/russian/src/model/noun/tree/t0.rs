use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['а', s @ ..] => super::t0::n1(s, out),
		['б', s @ ..] => super::t0::n2(s, out),
		['в', s @ ..] => super::t0::n3(s, out),
		['г', s @ ..] => super::t0::n4(s, out),
		['д', s @ ..] => super::t0::n5(s, out),
		['е', s @ ..] => super::t0::n6(s, out),
		['ж', s @ ..] => super::t0::n7(s, out),
		['з', s @ ..] => super::t0::n8(s, out),
		['и', s @ ..] => super::t0::n9(s, out),
		['й', s @ ..] => super::t0::n10(s, out),
		['к', s @ ..] => super::t0::n11(s, out),
		['л', s @ ..] => super::t0::n12(s, out),
		['м', s @ ..] => super::t0::n13(s, out),
		['н', s @ ..] => super::t0::n14(s, out),
		['о', s @ ..] => super::t0::n15(s, out),
		['п', s @ ..] => super::t0::n16(s, out),
		['р', s @ ..] => super::t0::n17(s, out),
		['с', s @ ..] => super::t0::n18(s, out),
		['т', s @ ..] => super::t0::n19(s, out),
		['у', s @ ..] => super::t0::n20(s, out),
		['ф', s @ ..] => super::t0::n21(s, out),
		['х', s @ ..] => super::t0::n22(s, out),
		['ц', s @ ..] => super::t0::n23(s, out),
		['ч', s @ ..] => super::t0::n24(s, out),
		['ш', s @ ..] => super::t0::n25(s, out),
		['щ', s @ ..] => {
			super::push(out, 26);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 505);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 4425);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 18376);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 42258);
											s.is_empty()
										}
										['л', s @ ..] => {
											super::push(out, 42259);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 18377);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 4426);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 18378);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 4427);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 18379);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 42260);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 4428);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 18380);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 42261);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 4429);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 18381);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 4430);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 18382);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 4431);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 18383);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 42262);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 70425);
													match s {
														[] => true,
														['с', s @ ..] => super::t0::n96617(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 506);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 4432);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 18384);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 507);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 4433);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 18385);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 4434);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18386);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 4435);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 18387);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 42263);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 70426);
													match s {
														[] => true,
														['т', s @ ..] => super::t0::n96618(s, out),
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
									super::push(out, 18388);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 42264);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 4436);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 18389);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 42265);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 4437);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 18390);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 42266);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['щ', s @ ..] => {
							super::push(out, 4438);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 18391);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 42267);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 70427);
													match s {
														[] => true,
														['к', s @ ..] => super::t0::n96619(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => super::t0::n509(s, out),
				['о', s @ ..] => super::t0::n510(s, out),
				['р', s @ ..] => super::t0::n511(s, out),
				['у', s @ ..] => super::t0::n512(s, out),
				['ы', s @ ..] => super::t0::n513(s, out),
				['ю', s @ ..] => super::t0::n514(s, out),
				['я', s @ ..] => super::t0::n515(s, out),
				_ => false,
			}
		}
		['ы', s @ ..] => super::t0::n27(s, out),
		['ь', s @ ..] => super::t0::n28(s, out),
		['э', s @ ..] => super::t0::n29(s, out),
		['ю', s @ ..] => super::t0::n30(s, out),
		['я', s @ ..] => super::t0::n31(s, out),
		_ => false,
	}
}

/// Words ending -а.
pub(crate) fn n1(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 32);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 599);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 600);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 4968);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 20010);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 45104);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ю', s @ ..] => {
							super::push(out, 4969);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 20011);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => super::t1::n33(s, out),
		['в', s @ ..] => super::t1::n34(s, out),
		['г', s @ ..] => super::t1::n35(s, out),
		['д', s @ ..] => super::t1::n36(s, out),
		['е', s @ ..] => {
			super::push(out, 37);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 686);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 687);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 5405);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 20854);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 45870);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 74237);
													match s {
														[] => true,
														['р', s @ ..] => super::t1::n99942(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 688);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 5406);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 20855);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 45871);
											s.is_empty()
										}
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
					super::push(out, 689);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 5407);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 20856);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 45872);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 74238);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 38);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 690);
					match s {
						[] => true,
						['г', s @ ..] => super::t1::n5420(s, out),
						_ => false,
					}
				}
				['а', s @ ..] => super::t1::n692(s, out),
				['б', s @ ..] => super::t1::n693(s, out),
				['д', s @ ..] => super::t1::n694(s, out),
				['е', s @ ..] => super::t1::n695(s, out),
				['ж', s @ ..] => super::t1::n696(s, out),
				['и', s @ ..] => super::t1::n697(s, out),
				['л', s @ ..] => super::t1::n698(s, out),
				['м', s @ ..] => super::t1::n699(s, out),
				['н', s @ ..] => super::t1::n700(s, out),
				['о', s @ ..] => super::t1::n701(s, out),
				['р', s @ ..] => super::t1::n702(s, out),
				['у', s @ ..] => super::t1::n703(s, out),
				['ы', s @ ..] => super::t1::n704(s, out),
				['ь', s @ ..] => super::t1::n705(s, out),
				['я', s @ ..] => super::t1::n706(s, out),
				_ => false,
			}
		}
		['з', s @ ..] => super::t1::n39(s, out),
		['и', s @ ..] => super::t1::n40(s, out),
		['к', s @ ..] => super::t1::n41(s, out),
		['л', s @ ..] => super::t1::n42(s, out),
		['м', s @ ..] => super::t1::n43(s, out),
		['н', s @ ..] => super::t1::n44(s, out),
		['о', s @ ..] => super::t1::n45(s, out),
		['п', s @ ..] => super::t1::n46(s, out),
		['р', s @ ..] => super::t1::n47(s, out),
		['с', s @ ..] => super::t1::n48(s, out),
		['т', s @ ..] => super::t1::n49(s, out),
		['у', s @ ..] => super::t1::n50(s, out),
		['ф', s @ ..] => super::t1::n51(s, out),
		['х', s @ ..] => super::t1::n52(s, out),
		['ц', s @ ..] => super::t1::n53(s, out),
		['ч', s @ ..] => super::t1::n54(s, out),
		['ш', s @ ..] => super::t1::n55(s, out),
		['щ', s @ ..] => super::t1::n56(s, out),
		['ъ', s @ ..] => super::t1::n57(s, out),
		['э', s @ ..] => super::t1::n58(s, out),
		_ => false,
	}
}

/// Words ending -б.
pub(crate) fn n2(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 59);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 1022);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 7044);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 7045);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 7046);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 25101);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 52415);
											s.is_empty()
										}
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
					super::push(out, 1023);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 7047);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 25102);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 7048);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 1024);
					s.is_empty()
				}
				['ж', 'д', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 7049);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 25103);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 52416);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 7050);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 25104);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 1025);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 7051);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 1026);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 7052);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 25105);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 52417);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 80997);
													s.is_empty()
												}
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
							super::push(out, 7053);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 25106);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 1027);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 7054);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 1028);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 7055);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 25107);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 52418);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 80998);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 25108);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 52419);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 80999);
													s.is_empty()
												}
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
							super::push(out, 7056);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 7057);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 25109);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 52420);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 81000);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1029);
					s.is_empty()
				}
				['р', s @ ..] => super::t1::n1032(s, out),
				['с', s @ ..] => {
					super::push(out, 1031);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 1032);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 7070);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 7071);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 7072);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 25121);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 52430);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 81006);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ш', s @ ..] => super::t1::n7093(s, out),
						_ => false,
					}
				}
				['х', s @ ..] => super::t1::n1035(s, out),
				_ => false,
			}
		}
		['б', s @ ..] => super::t1::n60(s, out),
		['е', s @ ..] => super::t1::n61(s, out),
		['з', s @ ..] => super::t1::n62(s, out),
		['и', s @ ..] => super::t1::n63(s, out),
		['й', s @ ..] => super::t1::n64(s, out),
		['л', s @ ..] => super::t1::n65(s, out),
		['м', s @ ..] => super::t1::n66(s, out),
		['о', s @ ..] => super::t1::n67(s, out),
		['р', s @ ..] => super::t1::n68(s, out),
		['у', s @ ..] => super::t1::n69(s, out),
		['ы', 'т', s @ ..] => super::t1::n70(s, out),
		['ь', s @ ..] => super::t1::n71(s, out),
		['э', s @ ..] => super::t1::n72(s, out),
		['ю', s @ ..] => super::t1::n73(s, out),
		['я', s @ ..] => super::t1::n74(s, out),
		_ => false,
	}
}

/// Words ending -в.
pub(crate) fn n3(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 74);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1101);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7219);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 1102);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 7220);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 1103);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 7221);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 25270);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 52526);
											match s {
												[] => true,
												['о', 'в', '-', s @ ..] => {
													super::push(out, 81055);
													match s {
														[] => true,
														['к', s @ ..] => super::t1::n105211(s, out),
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
									super::push(out, 25271);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 52527);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 81056);
													match s {
														[] => true,
														['г', s @ ..] => super::t1::n105212(s, out),
														_ => false,
													}
												}
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
							super::push(out, 7222);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 1104);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 7223);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 25272);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 52528);
											s.is_empty()
										}
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
					super::push(out, 1105);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 7224);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 7225);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 1106);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 7226);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 25273);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 52529);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => super::t1::n1109(s, out),
				['н', s @ ..] => {
					super::push(out, 1108);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 7235);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 25290);
									s.is_empty()
								}
								_ => false,
							}
						}
						['й', s @ ..] => {
							super::push(out, 7236);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 25291);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t1::n1111(s, out),
				['т', s @ ..] => super::t1::n1112(s, out),
				['у', s @ ..] => {
					super::push(out, 1111);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 7245);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 1112);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 7246);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 25301);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => super::t1::n1115(s, out),
				_ => false,
			}
		}
		['г', s @ ..] => super::t1::n76(s, out),
		['е', s @ ..] => super::t1::n77(s, out),
		['и', s @ ..] => super::t1::n78(s, out),
		['й', s @ ..] => super::t1::n79(s, out),
		['о', s @ ..] => super::t2::n80(s, out),
		['р', s @ ..] => super::t2::n81(s, out),
		['у', s @ ..] => super::t2::n82(s, out),
		['х', s @ ..] => super::t2::n83(s, out),
		['ы', s @ ..] => super::t2::n84(s, out),
		['ь', s @ ..] => super::t2::n85(s, out),
		['э', s @ ..] => super::t2::n86(s, out),
		['ю', s @ ..] => super::t2::n87(s, out),
		_ => false,
	}
}

/// Words ending -г.
pub(crate) fn n4(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 87);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1173);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 1174);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 7417);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 25519);
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
							super::push(out, 7418);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25520);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 7419);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 1175);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7420);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 7421);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 25521);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 52747);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 81199);
													match s {
														[] => true,
														['г', s @ ..] => super::t2::n105298(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1176);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7422);
							s.is_empty()
						}
						['а', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25522);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 25523);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 52748);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 81200);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => super::t2::n7445(s, out),
						['р', s @ ..] => {
							super::push(out, 7424);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 7425);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 7426);
							s.is_empty()
						}
						['ю', s @ ..] => super::t2::n7449(s, out),
						_ => false,
					}
				}
				['з', s @ ..] => super::t2::n1180(s, out),
				['л', s @ ..] => super::t2::n1181(s, out),
				['м', s @ ..] => super::t2::n1182(s, out),
				['н', s @ ..] => super::t2::n1183(s, out),
				['п', s @ ..] => super::t2::n1184(s, out),
				['р', s @ ..] => super::t2::n1185(s, out),
				['т', s @ ..] => super::t2::n1186(s, out),
				['ф', s @ ..] => super::t2::n1187(s, out),
				['ч', s @ ..] => super::t2::n1188(s, out),
				['ш', s @ ..] => super::t2::n1189(s, out),
				_ => false,
			}
		}
		['г', s @ ..] => super::t2::n89(s, out),
		['е', s @ ..] => super::t2::n90(s, out),
		['з', s @ ..] => super::t2::n91(s, out),
		['и', s @ ..] => super::t2::n92(s, out),
		['й', s @ ..] => super::t2::n93(s, out),
		['л', s @ ..] => super::t2::n94(s, out),
		['н', s @ ..] => super::t2::n95(s, out),
		['о', s @ ..] => super::t2::n96(s, out),
		['р', s @ ..] => super::t2::n97(s, out),
		['с', s @ ..] => super::t2::n98(s, out),
		['у', s @ ..] => super::t2::n99(s, out),
		['ы', s @ ..] => super::t2::n100(s, out),
		['э', s @ ..] => super::t2::n101(s, out),
		['ю', s @ ..] => super::t2::n102(s, out),
		['я', s @ ..] => super::t2::n103(s, out),
		_ => false,
	}
}

/// Words ending -д.
pub(crate) fn n5(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5);
	match s {
		[] => true,
		['а', s @ ..] => super::t2::n104(s, out),
		['б', s @ ..] => {
			super::push(out, 104);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1293);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7768);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 105);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1294);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 7769);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 26001);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 53185);
											s.is_empty()
										}
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
					super::push(out, 1295);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 7770);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26002);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t2::n107(s, out),
		['з', s @ ..] => {
			super::push(out, 107);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 1314);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 7836);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 26084);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 26085);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 7837);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 26086);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 53256);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 7838);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 26087);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 53257);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 7839);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 26088);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 53258);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 53259);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 81573);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['м', s @ ..] => {
									super::push(out, 26089);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 53260);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 81574);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 26090);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 53261);
											match s {
												[] => true,
												['ь', s @ ..] => {
													super::push(out, 81575);
													match s {
														[] => true,
														['л', s @ ..] => super::t2::n105517(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 53262);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 26091);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 53263);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 7840);
							s.is_empty()
						}
						['ъ', s @ ..] => {
							super::push(out, 7841);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26092);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 7842);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26093);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 53264);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 81576);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1315);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 7843);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 26094);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 26095);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 1316);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 7844);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t2::n109(s, out),
		['й', s @ ..] => {
			super::push(out, 109);
			match s {
				[] => true,
				['а', s @ ..] => super::t2::n1345(s, out),
				['е', s @ ..] => super::t2::n1346(s, out),
				['о', s @ ..] => {
					super::push(out, 1344);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 7957);
							s.is_empty()
						}
						['л', s @ ..] => super::t2::n7984(s, out),
						['н', s @ ..] => super::t2::n7985(s, out),
						_ => false,
					}
				}
				['э', s @ ..] => super::t2::n1348(s, out),
				_ => false,
			}
		}
		['л', s @ ..] => super::t2::n111(s, out),
		['н', s @ ..] => super::t2::n112(s, out),
		['о', s @ ..] => super::t2::n113(s, out),
		['р', s @ ..] => super::t2::n114(s, out),
		['у', s @ ..] => super::t2::n115(s, out),
		['ы', s @ ..] => super::t2::n116(s, out),
		['ь', s @ ..] => super::t2::n117(s, out),
		['э', s @ ..] => super::t2::n118(s, out),
		['ю', s @ ..] => super::t2::n119(s, out),
		['я', s @ ..] => super::t2::n120(s, out),
		_ => false,
	}
}

/// Words ending -е.
pub(crate) fn n6(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 120);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1411);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 8187);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 8188);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 26574);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 1412);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 8189);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 26575);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 1413);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 8190);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 8191);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 1414);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 8192);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 26576);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 53725);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ш', s @ ..] => {
									super::push(out, 26577);
									match s {
										[] => true,
										['ю', s @ ..] => {
											super::push(out, 53726);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 81924);
													s.is_empty()
												}
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
							super::push(out, 8193);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 26578);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 26579);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 53727);
											match s {
												[] => true,
												['й', s @ ..] => {
													super::push(out, 81925);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1415);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 8194);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 26580);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 53728);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 81926);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1416);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 8195);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 26581);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 53729);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 81927);
													match s {
														[] => true,
														['а', s @ ..] => super::t2::n105719(s, out),
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
									super::push(out, 26582);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 53730);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 121);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1417);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 8196);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 26583);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 53731);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 81928);
													s.is_empty()
												}
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
							super::push(out, 8197);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 26584);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 1418);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 8198);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 26585);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 53732);
											s.is_empty()
										}
										['н', s @ ..] => {
											super::push(out, 53733);
											s.is_empty()
										}
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
					super::push(out, 1419);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 8199);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 26586);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 53734);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 81929);
													match s {
														[] => true,
														['е', s @ ..] => super::t2::n105720(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1420);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 8200);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 26587);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 53735);
											match s {
												[] => true,
												['ш', s @ ..] => {
													super::push(out, 81930);
													s.is_empty()
												}
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
							super::push(out, 8201);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 26588);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 1421);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 8202);
							s.is_empty()
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 1422);
					s.is_empty()
				}
				['ю', s @ ..] => {
					super::push(out, 1423);
					match s {
						[] => true,
						['к', s @ ..] => super::t2::n8229(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => super::t2::n123(s, out),
		['д', s @ ..] => super::t2::n124(s, out),
		['е', s @ ..] => super::t2::n125(s, out),
		['ж', s @ ..] => super::t2::n126(s, out),
		['з', s @ ..] => super::t2::n127(s, out),
		['и', s @ ..] => super::t2::n128(s, out),
		['й', s @ ..] => super::t2::n129(s, out),
		['к', s @ ..] => super::t2::n130(s, out),
		['л', s @ ..] => super::t2::n131(s, out),
		['м', s @ ..] => super::t2::n132(s, out),
		['н', s @ ..] => super::t2::n133(s, out),
		['о', s @ ..] => super::t2::n134(s, out),
		['п', s @ ..] => super::t2::n135(s, out),
		['р', s @ ..] => super::t2::n136(s, out),
		['с', s @ ..] => super::t2::n137(s, out),
		['т', s @ ..] => super::t3::n138(s, out),
		['ф', s @ ..] => super::t3::n139(s, out),
		['х', s @ ..] => super::t3::n140(s, out),
		['ц', s @ ..] => super::t3::n141(s, out),
		['ч', s @ ..] => super::t3::n142(s, out),
		['ш', s @ ..] => super::t3::n143(s, out),
		['щ', s @ ..] => super::t3::n144(s, out),
		['ы', s @ ..] => super::t3::n145(s, out),
		['ь', s @ ..] => super::t3::n146(s, out),
		_ => false,
	}
}

/// Words ending -ж.
pub(crate) fn n7(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 146);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 1630);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8757);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 8758);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 27539);
									match s {
										[] => true,
										['\u{1}', s @ ..] => super::t3::n55704(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => super::t3::n1635(s, out),
				['г', s @ ..] => super::t3::n1636(s, out),
				['д', s @ ..] => super::t3::n1637(s, out),
				['з', s @ ..] => super::t3::n1638(s, out),
				['к', s @ ..] => super::t3::n1639(s, out),
				['л', s @ ..] => super::t3::n1640(s, out),
				['м', s @ ..] => super::t3::n1641(s, out),
				['н', s @ ..] => super::t3::n1642(s, out),
				['п', s @ ..] => super::t3::n1643(s, out),
				['р', s @ ..] => super::t3::n1644(s, out),
				['с', s @ ..] => super::t3::n1645(s, out),
				['т', s @ ..] => super::t3::n1646(s, out),
				['ф', s @ ..] => super::t3::n1647(s, out),
				['х', s @ ..] => super::t3::n1648(s, out),
				_ => false,
			}
		}
		['б', s @ ..] => super::t3::n148(s, out),
		['д', s @ ..] => super::t3::n149(s, out),
		['е', s @ ..] => super::t3::n150(s, out),
		['и', s @ ..] => super::t3::n151(s, out),
		['м', s @ ..] => super::t3::n152(s, out),
		['н', s @ ..] => super::t3::n153(s, out),
		['о', s @ ..] => super::t3::n154(s, out),
		['р', s @ ..] => super::t3::n155(s, out),
		['у', s @ ..] => super::t3::n156(s, out),
		['ы', s @ ..] => super::t3::n157(s, out),
		['я', s @ ..] => super::t3::n158(s, out),
		_ => false,
	}
}

/// Words ending -з.
pub(crate) fn n8(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 158);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1707);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 1708);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8931);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 8932);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 27705);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 8933);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 27706);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 27707);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 55256);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 83483);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1709);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8934);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 8935);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 1710);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 8936);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 27708);
									match s {
										[] => true,
										['\u{1}', s @ ..] => super::t3::n55829(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => super::t3::n1715(s, out),
				['з', s @ ..] => super::t3::n1716(s, out),
				['и', s @ ..] => super::t3::n1717(s, out),
				['к', s @ ..] => super::t3::n1718(s, out),
				['л', s @ ..] => super::t3::n1719(s, out),
				['м', s @ ..] => super::t3::n1720(s, out),
				['н', s @ ..] => super::t3::n1721(s, out),
				['п', s @ ..] => super::t3::n1722(s, out),
				['р', s @ ..] => super::t3::n1723(s, out),
				['т', s @ ..] => super::t3::n1724(s, out),
				['у', s @ ..] => super::t3::n1725(s, out),
				['х', s @ ..] => super::t3::n1726(s, out),
				_ => false,
			}
		}
		['б', s @ ..] => super::t3::n160(s, out),
		['д', s @ ..] => super::t3::n161(s, out),
		['е', s @ ..] => super::t3::n162(s, out),
		['з', s @ ..] => super::t3::n163(s, out),
		['и', s @ ..] => super::t3::n164(s, out),
		['й', s @ ..] => super::t3::n165(s, out),
		['л', s @ ..] => super::t3::n166(s, out),
		['н', s @ ..] => super::t3::n167(s, out),
		['о', s @ ..] => super::t3::n168(s, out),
		['р', s @ ..] => super::t3::n169(s, out),
		['у', s @ ..] => super::t3::n170(s, out),
		['ы', s @ ..] => super::t3::n171(s, out),
		['ь', s @ ..] => super::t3::n172(s, out),
		['э', s @ ..] => super::t3::n173(s, out),
		['ю', s @ ..] => super::t3::n174(s, out),
		['я', s @ ..] => super::t3::n175(s, out),
		_ => false,
	}
}

/// Words ending -и.
pub(crate) fn n9(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 175);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1808);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 1809);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 9155);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 27942);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 55443);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 9156);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 27943);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 1810);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 9157);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 27944);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 1811);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 9158);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 27945);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 55444);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 83603);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1812);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9159);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 176);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 1813);
					match s {
						[] => true,
						['н', s @ ..] => super::t3::n9187(s, out),
						_ => false,
					}
				}
				['а', s @ ..] => super::t3::n1820(s, out),
				['б', s @ ..] => super::t3::n1821(s, out),
				['г', s @ ..] => super::t3::n1822(s, out),
				['е', s @ ..] => {
					super::push(out, 1817);
					match s {
						[] => true,
						['б', s @ ..] => super::t3::n9201(s, out),
						_ => false,
					}
				}
				['ж', s @ ..] => super::t3::n1824(s, out),
				['з', s @ ..] => super::t3::n1825(s, out),
				['и', s @ ..] => super::t3::n1826(s, out),
				['й', s @ ..] => super::t3::n1827(s, out),
				['к', s @ ..] => super::t3::n1828(s, out),
				['л', s @ ..] => super::t3::n1829(s, out),
				['м', s @ ..] => super::t3::n1830(s, out),
				['н', s @ ..] => super::t3::n1831(s, out),
				['о', s @ ..] => super::t3::n1832(s, out),
				['р', s @ ..] => super::t3::n1833(s, out),
				['с', s @ ..] => super::t3::n1834(s, out),
				['т', s @ ..] => super::t3::n1835(s, out),
				['у', s @ ..] => super::t3::n1836(s, out),
				['ч', s @ ..] => super::t3::n1837(s, out),
				['ш', s @ ..] => super::t3::n1838(s, out),
				['ь', s @ ..] => super::t3::n1839(s, out),
				['я', s @ ..] => super::t3::n1840(s, out),
				_ => false,
			}
		}
		['в', s @ ..] => super::t3::n178(s, out),
		['г', s @ ..] => super::t3::n179(s, out),
		['д', s @ ..] => super::t3::n180(s, out),
		['е', s @ ..] => super::t3::n181(s, out),
		['ж', s @ ..] => super::t3::n182(s, out),
		['з', s @ ..] => super::t3::n183(s, out),
		['и', s @ ..] => super::t3::n184(s, out),
		['й', s @ ..] => super::t3::n185(s, out),
		['к', s @ ..] => super::t3::n186(s, out),
		['л', s @ ..] => super::t3::n187(s, out),
		['м', s @ ..] => super::t3::n188(s, out),
		['н', s @ ..] => super::t3::n189(s, out),
		['о', s @ ..] => super::t3::n190(s, out),
		['п', s @ ..] => super::t4::n191(s, out),
		['р', s @ ..] => super::t4::n192(s, out),
		['с', s @ ..] => super::t4::n193(s, out),
		['т', s @ ..] => super::t4::n194(s, out),
		['у', s @ ..] => super::t4::n195(s, out),
		['ф', s @ ..] => super::t4::n196(s, out),
		['х', s @ ..] => super::t4::n197(s, out),
		['ц', s @ ..] => super::t4::n198(s, out),
		['ч', s @ ..] => super::t4::n199(s, out),
		['ш', s @ ..] => super::t4::n200(s, out),
		['щ', s @ ..] => super::t4::n201(s, out),
		['ь', s @ ..] => super::t4::n202(s, out),
		_ => false,
	}
}

/// Words ending -й.
pub(crate) fn n10(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10);
	match s {
		[] => true,
		['а', s @ ..] => super::t4::n203(s, out),
		['е', s @ ..] => super::t4::n204(s, out),
		['и', s @ ..] => super::t4::n205(s, out),
		['о', s @ ..] => super::t4::n206(s, out),
		['у', s @ ..] => {
			super::push(out, 206);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2179);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 2180);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10435);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 10436);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 29748);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 57034);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 84786);
													s.is_empty()
												}
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
							super::push(out, 10437);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 29749);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 57035);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 10438);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 29750);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 57036);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 84787);
													match s {
														[] => true,
														['р', s @ ..] => super::t4::n107955(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 2181);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10439);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 2182);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 10440);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 29751);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 57037);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 84788);
													match s {
														[] => true,
														['н', s @ ..] => super::t4::n107956(s, out),
														_ => false,
													}
												}
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
							super::push(out, 10441);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 29752);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 57038);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 84789);
													s.is_empty()
												}
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
							super::push(out, 10442);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 29753);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 57039);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 84790);
													match s {
														[] => true,
														['м', s @ ..] => super::t4::n107957(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', 'т', 'е', 'в', '\u{1}', s @ ..] => {
									super::push(out, 29754);
									s.is_empty()
								}
								['т', s @ ..] => super::t4::n29962(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => super::t4::n2189(s, out),
				['з', s @ ..] => {
					super::push(out, 2184);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10447);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => super::t4::n2191(s, out),
				['л', s @ ..] => super::t4::n2192(s, out),
				['р', s @ ..] => super::t4::n2193(s, out),
				['с', s @ ..] => super::t4::n2194(s, out),
				['т', s @ ..] => super::t4::n2195(s, out),
				['х', s @ ..] => super::t4::n2196(s, out),
				['ч', s @ ..] => super::t4::n2197(s, out),
				['ш', s @ ..] => super::t4::n2198(s, out),
				_ => false,
			}
		}
		['ы', s @ ..] => super::t4::n208(s, out),
		['э', s @ ..] => super::t4::n209(s, out),
		['ю', s @ ..] => super::t4::n210(s, out),
		['я', s @ ..] => super::t4::n211(s, out),
		_ => false,
	}
}

/// Words ending -к.
pub(crate) fn n11(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 211);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 2216);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 10532);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 29895);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['а', s @ ..] => super::t4::n213(s, out),
		['в', s @ ..] => {
			super::push(out, 213);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2241);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 10707);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 30151);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 57404);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 85031);
													s.is_empty()
												}
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
							super::push(out, 10708);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 30152);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t4::n215(s, out),
		['и', s @ ..] => super::t4::n216(s, out),
		['й', s @ ..] => {
			super::push(out, 216);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2288);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 11078);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 31139);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 59271);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 87304);
													match s {
														[] => true,
														['а', s @ ..] => super::t4::n110148(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 2289);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 11079);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 31140);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 59272);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 217);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2290);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 2291);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 11080);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 31141);
									s.is_empty()
								}
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
			super::push(out, 218);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2292);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 11081);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 31142);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 59273);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 87305);
													match s {
														[] => true,
														['к', s @ ..] => super::t4::n110149(s, out),
														_ => false,
													}
												}
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
							super::push(out, 11082);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31143);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 2293);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 11083);
							s.is_empty()
						}
						['щ', s @ ..] => {
							super::push(out, 11084);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 31144);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 59274);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 87306);
													match s {
														[] => true,
														['п', s @ ..] => super::t4::n110150(s, out),
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
									super::push(out, 31145);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 59275);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 87307);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 31146);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 59276);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 31147);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 59277);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => super::t4::n2300(s, out),
				_ => false,
			}
		}
		['н', s @ ..] => super::t4::n220(s, out),
		['о', s @ ..] => super::t4::n221(s, out),
		['р', s @ ..] => super::t4::n222(s, out),
		['с', s @ ..] => super::t4::n223(s, out),
		['у', s @ ..] => super::t4::n224(s, out),
		['ц', s @ ..] => super::t4::n225(s, out),
		['ы', s @ ..] => super::t4::n226(s, out),
		['ь', s @ ..] => super::t4::n227(s, out),
		['э', s @ ..] => super::t4::n228(s, out),
		['ю', s @ ..] => super::t4::n229(s, out),
		['я', s @ ..] => super::t4::n230(s, out),
		_ => false,
	}
}

/// Words ending -л.
pub(crate) fn n12(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12);
	match s {
		[] => true,
		['а', s @ ..] => super::t4::n231(s, out),
		['б', s @ ..] => {
			super::push(out, 231);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 2451);
					s.is_empty()
				}
				['й', s @ ..] => {
					super::push(out, 2452);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 2453);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 232);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 233);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2454);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 11721);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32308);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 2455);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 2456);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 11722);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32309);
									s.is_empty()
								}
								['ж', s @ ..] => {
									super::push(out, 32310);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 60595);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 11723);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 32311);
									s.is_empty()
								}
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
			super::push(out, 234);
			s.is_empty()
		}
		['е', s @ ..] => super::t4::n236(s, out),
		['з', s @ ..] => {
			super::push(out, 236);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2475);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 11787);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 32393);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 60665);
											match s {
												[] => true,
												['ш', s @ ..] => {
													super::push(out, 88323);
													s.is_empty()
												}
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
							super::push(out, 11788);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 2476);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 11789);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32394);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t4::n238(s, out),
		['й', s @ ..] => {
			super::push(out, 238);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2498);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 11856);
							match s {
								[] => true,
								['с', 'и', s @ ..] => {
									super::push(out, 32466);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 60721);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 11857);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 2499);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 11858);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 32467);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 11859);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 11860);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 32468);
									match s {
										[] => true,
										['р', 'п', s @ ..] => {
											super::push(out, 60722);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 88361);
													s.is_empty()
												}
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
							super::push(out, 11861);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 32469);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 60723);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 88362);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t4::n110724(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 239);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2500);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 11862);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 32470);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 60724);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 11863);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 32471);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 60725);
											s.is_empty()
										}
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
					super::push(out, 2501);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 11864);
							match s {
								[] => true,
								['ю', s @ ..] => super::t4::n32759(s, out),
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 11865);
							match s {
								[] => true,
								['е', s @ ..] => super::t4::n32760(s, out),
								_ => false,
							}
						}
						['т', s @ ..] => super::t4::n11919(s, out),
						['ц', s @ ..] => {
							super::push(out, 11867);
							match s {
								[] => true,
								['о', s @ ..] => super::t4::n32762(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['й', s @ ..] => super::t4::n2511(s, out),
				['о', s @ ..] => super::t4::n2512(s, out),
				['э', s @ ..] => super::t4::n2513(s, out),
				_ => false,
			}
		}
		['л', s @ ..] => super::t4::n241(s, out),
		['о', s @ ..] => super::t4::n242(s, out),
		['п', s @ ..] => super::t4::n243(s, out),
		['р', s @ ..] => super::t4::n244(s, out),
		['с', s @ ..] => super::t5::n245(s, out),
		['т', s @ ..] => super::t5::n246(s, out),
		['у', s @ ..] => super::t5::n247(s, out),
		['ф', s @ ..] => super::t5::n248(s, out),
		['ы', s @ ..] => super::t5::n249(s, out),
		['э', s @ ..] => super::t5::n250(s, out),
		['я', s @ ..] => super::t5::n251(s, out),
		_ => false,
	}
}

/// Words ending -м.
pub(crate) fn n13(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 251);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2575);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12072);
							s.is_empty()
						}
						['л', s @ ..] => {
							match s {
								[] => true,
								['а', 'в', '\u{1}', s @ ..] => {
									super::push(out, 32704);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 32705);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 60892);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 88471);
													s.is_empty()
												}
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
							super::push(out, 12073);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 32706);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 60893);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 12074);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 2576);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 2577);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 12075);
							match s {
								[] => true,
								['и', s @ ..] => super::t5::n32994(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => super::t5::n2588(s, out),
				['д', s @ ..] => super::t5::n2589(s, out),
				['з', s @ ..] => super::t5::n2590(s, out),
				['и', s @ ..] => super::t5::n2591(s, out),
				['к', s @ ..] => super::t5::n2592(s, out),
				['л', s @ ..] => super::t5::n2593(s, out),
				['м', s @ ..] => super::t5::n2594(s, out),
				['н', s @ ..] => super::t5::n2595(s, out),
				['п', s @ ..] => super::t5::n2596(s, out),
				['р', s @ ..] => super::t5::n2597(s, out),
				['с', s @ ..] => super::t5::n2598(s, out),
				['т', s @ ..] => super::t5::n2599(s, out),
				['у', s @ ..] => super::t5::n2600(s, out),
				['х', s @ ..] => super::t5::n2601(s, out),
				['ч', s @ ..] => super::t5::n2602(s, out),
				['ш', s @ ..] => super::t5::n2603(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t5::n253(s, out),
		['з', s @ ..] => super::t5::n254(s, out),
		['и', s @ ..] => super::t5::n255(s, out),
		['й', s @ ..] => super::t5::n256(s, out),
		['л', s @ ..] => super::t5::n257(s, out),
		['м', s @ ..] => super::t5::n258(s, out),
		['н', s @ ..] => super::t5::n259(s, out),
		['о', s @ ..] => super::t5::n260(s, out),
		['р', s @ ..] => super::t5::n261(s, out),
		['с', s @ ..] => super::t5::n262(s, out),
		['т', s @ ..] => super::t5::n263(s, out),
		['у', s @ ..] => super::t5::n264(s, out),
		['ф', s @ ..] => super::t5::n265(s, out),
		['ы', s @ ..] => super::t5::n266(s, out),
		['ь', s @ ..] => super::t5::n267(s, out),
		['э', s @ ..] => super::t5::n268(s, out),
		['ю', s @ ..] => super::t5::n269(s, out),
		['я', s @ ..] => super::t5::n270(s, out),
		_ => false,
	}
}

/// Words ending -н.
pub(crate) fn n14(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 270);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 2706);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 2707);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 12473);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 33196);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 61295);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['а', s @ ..] => super::t5::n272(s, out),
		['в', s @ ..] => {
			super::push(out, 272);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2732);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 12732);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 33734);
									s.is_empty()
								}
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
			super::push(out, 273);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2733);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 12733);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 33735);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t5::n275(s, out),
		['ж', s @ ..] => {
			super::push(out, 275);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 2759);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 12902);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 33998);
									s.is_empty()
								}
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
			super::push(out, 276);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2760);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 12903);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 33999);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t5::n278(s, out),
		['й', s @ ..] => {
			super::push(out, 278);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2787);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13123);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 13124);
							s.is_empty()
						}
						['ж', s @ ..] => {
							super::push(out, 13125);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 34409);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 62527);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 13126);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 34410);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 13127);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 34411);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 62528);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 89536);
													match s {
														[] => true,
														['д', s @ ..] => super::t5::n111368(s, out),
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
									super::push(out, 34412);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 62529);
											match s {
												[] => true,
												['э', s @ ..] => {
													super::push(out, 89537);
													match s {
														[] => true,
														['к', s @ ..] => super::t5::n111369(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 2788);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 13128);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 34413);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 13129);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 34414);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 62530);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 89538);
													match s {
														[] => true,
														['ф', s @ ..] => super::t5::n111370(s, out),
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
									super::push(out, 34415);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 62531);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 13130);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 13131);
							match s {
								[] => true,
								['с', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 62532);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 89539);
													s.is_empty()
												}
												_ => false,
											}
										}
										['у', s @ ..] => super::t5::n63189(s, out),
										_ => false,
									}
								}
								['у', s @ ..] => super::t5::n34748(s, out),
								_ => false,
							}
						}
						['т', s @ ..] => super::t5::n13203(s, out),
						['ф', s @ ..] => super::t5::n13204(s, out),
						['ч', s @ ..] => super::t5::n13205(s, out),
						_ => false,
					}
				}
				['о', s @ ..] => super::t5::n2799(s, out),
				['э', s @ ..] => super::t5::n2800(s, out),
				['я', s @ ..] => super::t5::n2801(s, out),
				_ => false,
			}
		}
		['л', s @ ..] => super::t5::n280(s, out),
		['м', s @ ..] => super::t5::n281(s, out),
		['н', s @ ..] => super::t5::n282(s, out),
		['о', s @ ..] => super::t5::n283(s, out),
		['р', s @ ..] => super::t5::n284(s, out),
		['у', s @ ..] => super::t5::n285(s, out),
		['ш', s @ ..] => super::t5::n286(s, out),
		['ы', s @ ..] => super::t5::n287(s, out),
		['ь', s @ ..] => super::t5::n288(s, out),
		['э', s @ ..] => super::t5::n289(s, out),
		['ю', s @ ..] => super::t5::n290(s, out),
		['я', s @ ..] => super::t5::n291(s, out),
		_ => false,
	}
}

/// Words ending -о.
pub(crate) fn n15(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 291);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 2899);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 13558);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 35109);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 63135);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 89927);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 2900);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 13559);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 35110);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 63136);
											s.is_empty()
										}
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
					super::push(out, 2901);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13560);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 35111);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 35112);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 2902);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13561);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 35113);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 63137);
											s.is_empty()
										}
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
					super::push(out, 2903);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 13562);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 35114);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 2904);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13563);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 35115);
									match s {
										[] => true,
										['ю', s @ ..] => {
											super::push(out, 63138);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 89928);
													s.is_empty()
												}
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
							super::push(out, 13564);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 35116);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 63139);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 89929);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 2905);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13565);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 35117);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 63140);
											s.is_empty()
										}
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
					super::push(out, 2906);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13566);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 2907);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13567);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 13568);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 13569);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 13570);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 292);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 2908);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 13571);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 35118);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 63141);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['а', s @ ..] => super::t5::n2919(s, out),
				['б', s @ ..] => {
					super::push(out, 2910);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 13578);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 35121);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 2911);
					match s {
						[] => true,
						['и', s @ ..] => super::t5::n13653(s, out),
						_ => false,
					}
				}
				['е', s @ ..] => super::t5::n2922(s, out),
				['и', s @ ..] => super::t5::n2923(s, out),
				['й', s @ ..] => super::t5::n2924(s, out),
				['м', s @ ..] => super::t5::n2925(s, out),
				['о', s @ ..] => super::t5::n2926(s, out),
				['р', s @ ..] => super::t5::n2927(s, out),
				_ => false,
			}
		}
		['в', s @ ..] => super::t5::n294(s, out),
		['г', s @ ..] => super::t5::n295(s, out),
		['д', s @ ..] => super::t5::n296(s, out),
		['е', s @ ..] => super::t5::n297(s, out),
		['ж', s @ ..] => super::t5::n298(s, out),
		['з', s @ ..] => super::t5::n299(s, out),
		['и', s @ ..] => super::t5::n300(s, out),
		['й', s @ ..] => super::t5::n301(s, out),
		['к', s @ ..] => super::t5::n302(s, out),
		['л', s @ ..] => super::t5::n303(s, out),
		['м', s @ ..] => super::t5::n304(s, out),
		['н', s @ ..] => super::t5::n305(s, out),
		['о', s @ ..] => super::t5::n306(s, out),
		['п', s @ ..] => super::t5::n307(s, out),
		['р', s @ ..] => super::t6::n308(s, out),
		['с', s @ ..] => super::t6::n309(s, out),
		['т', s @ ..] => super::t6::n310(s, out),
		['у', s @ ..] => super::t6::n311(s, out),
		['ф', s @ ..] => super::t6::n312(s, out),
		['х', s @ ..] => super::t6::n313(s, out),
		['ц', s @ ..] => super::t6::n314(s, out),
		['ч', s @ ..] => super::t6::n315(s, out),
		['ш', s @ ..] => super::t6::n316(s, out),
		['ь', s @ ..] => super::t6::n317(s, out),
		['я', s @ ..] => super::t6::n318(s, out),
		_ => false,
	}
}

/// Words ending -п.
pub(crate) fn n16(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 318);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 3178);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 3179);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 14341);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 36229);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 3180);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 3181);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 14342);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 36230);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 64197);
											s.is_empty()
										}
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
					super::push(out, 3182);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 14343);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 14344);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 36231);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 3183);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 14345);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 36232);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 64198);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 14346);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 36233);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 64199);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 14347);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 36234);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 64200);
											match s {
												[] => true,
												['э', s @ ..] => {
													super::push(out, 90823);
													s.is_empty()
												}
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
							super::push(out, 14348);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 3184);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 14349);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 36235);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 64201);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['й', s @ ..] => {
							super::push(out, 14350);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 36236);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 64202);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 90824);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 3185);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 14351);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 36237);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 14352);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 36238);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 14353);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 36239);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 64203);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 90825);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 36240);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 14354);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 36241);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 36242);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 64204);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 90826);
													s.is_empty()
												}
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
							super::push(out, 14355);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 36243);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 64205);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 14356);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 3186);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 3187);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 14357);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 36244);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 64206);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 14358);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 14359);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 36245);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 64207);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['э', s @ ..] => {
							super::push(out, 14360);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 3188);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 14361);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 36246);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 64208);
											s.is_empty()
										}
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
					super::push(out, 3189);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t6::n320(s, out),
		['и', s @ ..] => super::t6::n321(s, out),
		['й', s @ ..] => {
			super::push(out, 321);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3215);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 3216);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => super::t6::n323(s, out),
		['м', s @ ..] => super::t6::n324(s, out),
		['о', s @ ..] => super::t6::n325(s, out),
		['п', s @ ..] => super::t6::n326(s, out),
		['р', s @ ..] => super::t6::n327(s, out),
		['у', s @ ..] => super::t6::n328(s, out),
		['ы', s @ ..] => super::t6::n329(s, out),
		['ь', s @ ..] => {
			super::push(out, 329);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 3257);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 330);
			match s {
				[] => true,
				['к', s @ ..] => super::t6::n3269(s, out),
				['р', s @ ..] => super::t6::n3270(s, out),
				_ => false,
			}
		}
		['ю', s @ ..] => super::t6::n332(s, out),
		['я', s @ ..] => super::t6::n333(s, out),
		_ => false,
	}
}

/// Words ending -р.
pub(crate) fn n17(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 333);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 3262);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 3263);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 14504);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['а', s @ ..] => super::t6::n335(s, out),
		['б', s @ ..] => {
			super::push(out, 335);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3290);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 14675);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 14676);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 36627);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 64506);
											s.is_empty()
										}
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
					super::push(out, 3291);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 14677);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 3292);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 14678);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 3293);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 14679);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 36628);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 3294);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14680);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 14681);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 36629);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 14682);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 36630);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 64507);
											s.is_empty()
										}
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
					super::push(out, 3295);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 14683);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 3296);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 14684);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 36631);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 3297);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 14685);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 36632);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 3298);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 14686);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 36633);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 64508);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 91000);
													s.is_empty()
												}
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
							super::push(out, 14687);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 36634);
									s.is_empty()
								}
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
			super::push(out, 336);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3299);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 14688);
							match s {
								[] => true,
								['о', 'л', s @ ..] => {
									super::push(out, 36635);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 64509);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 91001);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t6::n112401(s, out),
														_ => false,
													}
												}
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
							super::push(out, 14689);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 36636);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 14690);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 36637);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 36638);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 64510);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 14691);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 36639);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 36640);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 64511);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 36641);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 64512);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 91002);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 3300);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 14692);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 36642);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 64513);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 14693);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 36643);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 64514);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 14694);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 3301);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 14695);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 36644);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 3302);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 14696);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 36645);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => super::t6::n338(s, out),
		['д', s @ ..] => super::t6::n339(s, out),
		['е', s @ ..] => super::t6::n340(s, out),
		['и', s @ ..] => super::t6::n341(s, out),
		['й', s @ ..] => super::t6::n342(s, out),
		['к', s @ ..] => super::t6::n343(s, out),
		['м', s @ ..] => {
			super::push(out, 343);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 3377);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 15099);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => super::t6::n345(s, out),
		['о', s @ ..] => super::t6::n346(s, out),
		['п', s @ ..] => super::t6::n347(s, out),
		['р', s @ ..] => super::t6::n348(s, out),
		['с', s @ ..] => super::t6::n349(s, out),
		['т', s @ ..] => super::t6::n350(s, out),
		['у', s @ ..] => super::t6::n351(s, out),
		['ф', s @ ..] => super::t6::n352(s, out),
		['х', s @ ..] => super::t6::n353(s, out),
		['ы', s @ ..] => super::t6::n354(s, out),
		['э', s @ ..] => super::t6::n355(s, out),
		['ю', s @ ..] => super::t6::n356(s, out),
		['я', s @ ..] => super::t6::n357(s, out),
		_ => false,
	}
}

/// Words ending -с.
pub(crate) fn n18(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18);
	match s {
		[] => true,
		['а', s @ ..] => super::t6::n358(s, out),
		['б', s @ ..] => {
			super::push(out, 358);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 3502);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 15569);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 38205);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 3503);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 15570);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 3504);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 15571);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 38206);
									s.is_empty()
								}
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
			super::push(out, 359);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 3505);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 15572);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 15573);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 38207);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 3506);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 15574);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 38208);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 66342);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 92472);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 360);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3507);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 15575);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 3508);
					match s {
						[] => true,
						['и', s @ ..] => {
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 38209);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 66343);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 92473);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 38210);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 66344);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 92474);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 3509);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 15576);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 38211);
									s.is_empty()
								}
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
			super::push(out, 361);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 3510);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 15577);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 38212);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 66345);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 92475);
													match s {
														[] => true,
														['в', s @ ..] => super::t6::n113424(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 3511);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 15578);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 3512);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 15579);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 38213);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 66346);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 92476);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 3513);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 15580);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t6::n363(s, out),
		['и', s @ ..] => super::t6::n364(s, out),
		['й', s @ ..] => {
			super::push(out, 364);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3557);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 15753);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 15754);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 38458);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 66564);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 92610);
													match s {
														[] => true,
														['о', s @ ..] => super::t6::n113511(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 3558);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 3559);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => super::t6::n366(s, out),
		['л', s @ ..] => {
			super::push(out, 366);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3569);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 15804);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 3570);
					match s {
						[] => true,
						['п', s @ ..] => super::t6::n15892(s, out),
						['т', s @ ..] => super::t6::n15893(s, out),
						_ => false,
					}
				}
				['и', s @ ..] => super::t6::n3582(s, out),
				['л', s @ ..] => super::t6::n3583(s, out),
				['у', s @ ..] => super::t6::n3584(s, out),
				['э', s @ ..] => super::t6::n3585(s, out),
				_ => false,
			}
		}
		['м', s @ ..] => super::t6::n368(s, out),
		['н', s @ ..] => super::t6::n369(s, out),
		['о', s @ ..] => super::t6::n370(s, out),
		['п', s @ ..] => super::t6::n371(s, out),
		['р', s @ ..] => super::t6::n372(s, out),
		['с', s @ ..] => super::t6::n373(s, out),
		['т', s @ ..] => super::t6::n374(s, out),
		['у', s @ ..] => super::t6::n375(s, out),
		['х', s @ ..] => super::t6::n376(s, out),
		['ы', s @ ..] => super::t6::n377(s, out),
		['ь', s @ ..] => super::t7::n378(s, out),
		['э', s @ ..] => super::t7::n379(s, out),
		['ю', s @ ..] => super::t7::n380(s, out),
		['я', s @ ..] => super::t7::n381(s, out),
		_ => false,
	}
}

/// Words ending -т.
pub(crate) fn n19(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 381);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 3674);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16218);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['а', s @ ..] => super::t7::n383(s, out),
		['б', s @ ..] => {
			super::push(out, 383);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3697);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 384);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3698);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 16386);
							match s {
								[] => true,
								['о', 'м', s @ ..] => {
									super::push(out, 39419);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 67481);
											match s {
												[] => true,
												['о', 'к', '-', s @ ..] => {
													super::push(out, 93210);
													match s {
														[] => true,
														['а', s @ ..] => super::t7::n113841(s, out),
														['к', s @ ..] => super::t7::n113842(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 3699);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 16387);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 3700);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 16388);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 39420);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 67482);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 93211);
													match s {
														[] => true,
														['п', s @ ..] => super::t7::n113843(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 385);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 3701);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 16389);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 386);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3702);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 16390);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 39421);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 67483);
											match s {
												[] => true,
												['й', s @ ..] => {
													super::push(out, 93212);
													match s {
														[] => true,
														['а', s @ ..] => super::t7::n113844(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 67484);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 93213);
													match s {
														[] => true,
														['т', s @ ..] => super::t7::n113845(s, out),
														_ => false,
													}
												}
												['о', s @ ..] => super::t7::n94213(s, out),
												_ => false,
											}
										}
										['р', s @ ..] => super::t7::n68183(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => super::t7::n3714(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t7::n388(s, out),
		['и', s @ ..] => super::t7::n389(s, out),
		['й', s @ ..] => super::t7::n390(s, out),
		['к', s @ ..] => super::t7::n391(s, out),
		['л', s @ ..] => super::t7::n392(s, out),
		['м', s @ ..] => super::t7::n393(s, out),
		['н', s @ ..] => super::t7::n394(s, out),
		['о', s @ ..] => super::t7::n395(s, out),
		['п', s @ ..] => super::t7::n396(s, out),
		['р', s @ ..] => super::t7::n397(s, out),
		['с', s @ ..] => super::t7::n398(s, out),
		['т', s @ ..] => super::t7::n399(s, out),
		['у', s @ ..] => super::t7::n400(s, out),
		['ф', s @ ..] => super::t7::n401(s, out),
		['х', s @ ..] => super::t7::n402(s, out),
		['ч', s @ ..] => super::t7::n403(s, out),
		['ш', s @ ..] => super::t7::n404(s, out),
		['ы', s @ ..] => super::t7::n405(s, out),
		['ь', s @ ..] => super::t7::n406(s, out),
		['э', s @ ..] => super::t7::n407(s, out),
		['ю', s @ ..] => super::t7::n408(s, out),
		['я', s @ ..] => super::t7::n409(s, out),
		_ => false,
	}
}

/// Words ending -у.
pub(crate) fn n20(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 409);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 3887);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 17203);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 40747);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 68779);
											s.is_empty()
										}
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
					super::push(out, 3888);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 17204);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 17205);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 40748);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 68780);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 68781);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 94213);
													s.is_empty()
												}
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
							super::push(out, 17206);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 40749);
									match s {
										[] => true,
										['й', s @ ..] => {
											super::push(out, 68782);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ц', s @ ..] => {
							super::push(out, 17207);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 40750);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 68783);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 94214);
													s.is_empty()
												}
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
							super::push(out, 17208);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 40751);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 68784);
											s.is_empty()
										}
										['и', s @ ..] => {
											super::push(out, 68785);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 94215);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 3889);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 17209);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 40752);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 68786);
											s.is_empty()
										}
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
					super::push(out, 3890);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 17210);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 3891);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 17211);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 40753);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 40754);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 17212);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 40755);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 3892);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 17213);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 40756);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 3893);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 17214);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 40757);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 17215);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 40758);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 68787);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 40759);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 68788);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 94216);
													match s {
														[] => true,
														['ю', s @ ..] => super::t7::n114495(s, out),
														_ => false,
													}
												}
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
							super::push(out, 17216);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 40760);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 3894);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 17217);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 40761);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 17218);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 17219);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 40762);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 68789);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => super::t7::n3906(s, out),
				['т', s @ ..] => super::t7::n3907(s, out),
				['ч', s @ ..] => super::t7::n3908(s, out),
				['ш', s @ ..] => super::t7::n3909(s, out),
				_ => false,
			}
		}
		['б', s @ ..] => super::t7::n411(s, out),
		['в', s @ ..] => super::t7::n412(s, out),
		['г', s @ ..] => super::t7::n413(s, out),
		['д', s @ ..] => super::t7::n414(s, out),
		['ж', s @ ..] => super::t7::n415(s, out),
		['з', s @ ..] => super::t7::n416(s, out),
		['и', s @ ..] => super::t7::n417(s, out),
		['к', s @ ..] => super::t7::n418(s, out),
		['л', s @ ..] => super::t7::n419(s, out),
		['м', s @ ..] => super::t7::n420(s, out),
		['н', s @ ..] => super::t7::n421(s, out),
		['о', s @ ..] => super::t7::n422(s, out),
		['п', s @ ..] => super::t7::n423(s, out),
		['р', s @ ..] => super::t7::n424(s, out),
		['с', s @ ..] => super::t7::n425(s, out),
		['т', s @ ..] => super::t7::n426(s, out),
		['у', s @ ..] => super::t7::n427(s, out),
		['ф', s @ ..] => super::t7::n428(s, out),
		['х', s @ ..] => super::t7::n429(s, out),
		['ц', s @ ..] => super::t7::n430(s, out),
		['ч', s @ ..] => super::t7::n431(s, out),
		['ш', s @ ..] => super::t7::n432(s, out),
		['я', s @ ..] => super::t7::n433(s, out),
		_ => false,
	}
}

/// Words ending -ф.
pub(crate) fn n21(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21);
	match s {
		[] => true,
		['а', s @ ..] => super::t7::n434(s, out),
		['е', s @ ..] => {
			super::push(out, 434);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 4030);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 17430);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 17431);
							match s {
								[] => true,
								['й', s @ ..] => {
									super::push(out, 40963);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 68947);
											s.is_empty()
										}
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
					super::push(out, 4031);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 17432);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 40964);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 68948);
											s.is_empty()
										}
										['д', s @ ..] => {
											super::push(out, 68949);
											s.is_empty()
										}
										_ => false,
									}
								}
								['й', s @ ..] => {
									super::push(out, 40965);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ю', s @ ..] => {
							super::push(out, 17433);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 40966);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 4032);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 17434);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 17435);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 4033);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17436);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 4034);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 17437);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 40967);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 68950);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 94352);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 4035);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17438);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17439);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 40968);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 68951);
											match s {
												[] => true,
												['ц', s @ ..] => {
													super::push(out, 94353);
													s.is_empty()
												}
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
							super::push(out, 17440);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 17441);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 4036);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17442);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 4037);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 17443);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 40969);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 17444);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 40970);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 4038);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 17445);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 40971);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t7::n436(s, out),
		['й', s @ ..] => {
			super::push(out, 436);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4055);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 17475);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 17476);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 17477);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 41007);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 68974);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 94374);
													match s {
														[] => true,
														['л', s @ ..] => super::t7::n114608(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 4056);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 437);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4057);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 17478);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 438);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 4058);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 17479);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 41008);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 68975);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 94375);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 439);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 4059);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 17480);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 41009);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 68976);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 440);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4060);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 17481);
							match s {
								[] => true,
								['а', s @ ..] => super::t7::n41423(s, out),
								_ => false,
							}
						}
						['с', s @ ..] => super::t7::n17581(s, out),
						_ => false,
					}
				}
				['и', s @ ..] => super::t7::n4072(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t7::n442(s, out),
		['п', s @ ..] => super::t7::n443(s, out),
		['р', s @ ..] => super::t7::n444(s, out),
		['у', s @ ..] => super::t7::n445(s, out),
		['ф', s @ ..] => super::t7::n446(s, out),
		['ь', s @ ..] => super::t7::n447(s, out),
		['э', s @ ..] => super::t7::n448(s, out),
		['ю', s @ ..] => super::t7::n449(s, out),
		_ => false,
	}
}

/// Words ending -х.
pub(crate) fn n22(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 449);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 4092);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17536);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17537);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 41061);
									s.is_empty()
								}
								_ => false,
							}
						}
						['й', s @ ..] => {
							super::push(out, 17538);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 41062);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 69023);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 94404);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 4093);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 17539);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 41063);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 17540);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 41064);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 69024);
											s.is_empty()
										}
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
					super::push(out, 4094);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 17541);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 41065);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => super::t7::n17641(s, out),
						_ => false,
					}
				}
				['к', s @ ..] => super::t7::n4106(s, out),
				['л', s @ ..] => super::t7::n4107(s, out),
				['м', s @ ..] => super::t7::n4108(s, out),
				['н', s @ ..] => super::t7::n4109(s, out),
				['п', s @ ..] => super::t8::n4110(s, out),
				['р', s @ ..] => super::t8::n4111(s, out),
				['с', s @ ..] => super::t8::n4112(s, out),
				['т', s @ ..] => super::t8::n4113(s, out),
				['х', s @ ..] => super::t8::n4114(s, out),
				['ц', s @ ..] => super::t8::n4115(s, out),
				['ш', s @ ..] => super::t8::n4116(s, out),
				_ => false,
			}
		}
		['в', s @ ..] => super::t8::n451(s, out),
		['г', s @ ..] => super::t8::n452(s, out),
		['д', s @ ..] => super::t8::n453(s, out),
		['е', s @ ..] => super::t8::n454(s, out),
		['и', s @ ..] => super::t8::n455(s, out),
		['й', s @ ..] => super::t8::n456(s, out),
		['к', s @ ..] => super::t8::n457(s, out),
		['л', s @ ..] => super::t8::n458(s, out),
		['н', s @ ..] => super::t8::n459(s, out),
		['о', s @ ..] => super::t8::n460(s, out),
		['р', s @ ..] => super::t8::n461(s, out),
		['с', s @ ..] => super::t8::n462(s, out),
		['т', s @ ..] => super::t8::n463(s, out),
		['у', s @ ..] => super::t8::n464(s, out),
		['ы', s @ ..] => super::t8::n465(s, out),
		['ю', s @ ..] => super::t8::n466(s, out),
		['я', s @ ..] => super::t8::n467(s, out),
		_ => false,
	}
}

/// Words ending -ц.
pub(crate) fn n23(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 467);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 4186);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 17734);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 41264);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 69180);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 94487);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 4187);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 17735);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 41265);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 17736);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 41266);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 69181);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 17737);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 41267);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 69182);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 94488);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['э', s @ ..] => {
									super::push(out, 41268);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 17738);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 41269);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 69183);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 94489);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 4188);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17739);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 4189);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 17740);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 41270);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 17741);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 41271);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 69184);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 17742);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 17743);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 41272);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 17744);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 41273);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 69185);
											s.is_empty()
										}
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
					super::push(out, 4190);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17745);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 17746);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 41274);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 4191);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 17747);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 41275);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 69186);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 17748);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 41276);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 69187);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 94490);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 4192);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 17749);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 41277);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 69188);
											s.is_empty()
										}
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
					super::push(out, 4193);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17750);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t8::n469(s, out),
		['и', s @ ..] => {
			super::push(out, 469);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 4219);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 17944);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 41826);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 70102);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 95401);
													match s {
														[] => true,
														['к', s @ ..] => super::t8::n115310(s, out),
														_ => false,
													}
												}
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
							super::push(out, 17945);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 4220);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 17946);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => super::t8::n4232(s, out),
				['м', s @ ..] => {
					super::push(out, 4222);
					match s {
						[] => true,
						['а', s @ ..] => super::t8::n18051(s, out),
						_ => false,
					}
				}
				['п', s @ ..] => super::t8::n4234(s, out),
				['р', s @ ..] => super::t8::n4235(s, out),
				_ => false,
			}
		}
		['й', s @ ..] => super::t8::n471(s, out),
		['н', s @ ..] => super::t8::n472(s, out),
		['о', s @ ..] => super::t8::n473(s, out),
		['р', s @ ..] => super::t8::n474(s, out),
		['т', s @ ..] => super::t8::n475(s, out),
		['у', s @ ..] => super::t8::n476(s, out),
		['ь', s @ ..] => super::t8::n477(s, out),
		['э', s @ ..] => super::t8::n478(s, out),
		['я', s @ ..] => super::t8::n479(s, out),
		_ => false,
	}
}

/// Words ending -ч.
pub(crate) fn n24(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24);
	match s {
		[] => true,
		['а', s @ ..] => super::t8::n480(s, out),
		['е', s @ ..] => {
			super::push(out, 480);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 4260);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 18059);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 41930);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 18060);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 41931);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 4261);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 18061);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 41932);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 4262);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18062);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 4263);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 18063);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 41933);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 70162);
											s.is_empty()
										}
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
					super::push(out, 4264);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 18064);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 41934);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 70163);
											match s {
												[] => true,
												['ы', s @ ..] => {
													super::push(out, 95433);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 4265);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 18065);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 41935);
									s.is_empty()
								}
								_ => false,
							}
						}
						['й', s @ ..] => {
							super::push(out, 18066);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 41936);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 18067);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 41937);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 4266);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 18068);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 41938);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t8::n482(s, out),
		['й', s @ ..] => {
			super::push(out, 482);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 4286);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 18115);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 42001);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 70238);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 95499);
													match s {
														[] => true,
														['е', s @ ..] => super::t8::n115381(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 483);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4287);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 18116);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 42002);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 70239);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 95500);
													s.is_empty()
												}
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
							super::push(out, 18117);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 18118);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 42003);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 70240);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 95501);
													s.is_empty()
												}
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
							super::push(out, 18119);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 4288);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 18120);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 42004);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 70241);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 95502);
													match s {
														[] => true,
														['а', s @ ..] => super::t8::n115382(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 4289);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18121);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 42005);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 18122);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 484);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 4290);
					s.is_empty()
				}
				['м', s @ ..] => super::t8::n4302(s, out),
				['р', s @ ..] => super::t8::n4303(s, out),
				['т', s @ ..] => super::t8::n4304(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => super::t8::n486(s, out),
		['т', s @ ..] => super::t8::n487(s, out),
		['у', s @ ..] => super::t8::n488(s, out),
		['ы', s @ ..] => super::t8::n489(s, out),
		['ь', s @ ..] => super::t8::n490(s, out),
		['ю', s @ ..] => super::t8::n491(s, out),
		['я', s @ ..] => super::t8::n492(s, out),
		_ => false,
	}
}

/// Words ending -ш.
pub(crate) fn n25(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 492);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 4331);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 18171);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 42051);
									s.is_empty()
								}
								['л', 'а', 'к', '\u{1}', s @ ..] => {
									super::push(out, 42052);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 42053);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 70270);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 95522);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ш', s @ ..] => {
									super::push(out, 42054);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 70271);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 18172);
							match s {
								[] => true,
								['ъ', s @ ..] => {
									super::push(out, 42055);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 70272);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 95523);
													match s {
														[] => true,
														['п', s @ ..] => super::t8::n115388(s, out),
														_ => false,
													}
												}
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
							super::push(out, 18173);
							match s {
								[] => true,
								['л', s @ ..] => super::t8::n42477(s, out),
								_ => false,
							}
						}
						['к', s @ ..] => super::t8::n18275(s, out),
						['л', s @ ..] => super::t8::n18276(s, out),
						['о', s @ ..] => super::t8::n18277(s, out),
						['р', s @ ..] => super::t8::n18278(s, out),
						['ь', s @ ..] => super::t8::n18279(s, out),
						_ => false,
					}
				}
				['в', s @ ..] => super::t8::n4343(s, out),
				['г', s @ ..] => super::t8::n4344(s, out),
				['д', s @ ..] => super::t8::n4345(s, out),
				['к', s @ ..] => super::t8::n4346(s, out),
				['л', s @ ..] => super::t8::n4347(s, out),
				['м', s @ ..] => super::t8::n4348(s, out),
				['н', s @ ..] => super::t8::n4349(s, out),
				['п', s @ ..] => super::t8::n4350(s, out),
				['р', s @ ..] => super::t8::n4351(s, out),
				['т', s @ ..] => super::t8::n4352(s, out),
				['х', s @ ..] => {
					super::push(out, 4342);
					s.is_empty()
				}
				['ш', s @ ..] => super::t8::n4354(s, out),
				['я', s @ ..] => super::t8::n4355(s, out),
				_ => false,
			}
		}
		['в', s @ ..] => super::t8::n494(s, out),
		['е', s @ ..] => super::t8::n495(s, out),
		['и', s @ ..] => super::t8::n496(s, out),
		['н', s @ ..] => super::t8::n497(s, out),
		['о', s @ ..] => super::t8::n498(s, out),
		['п', s @ ..] => super::t8::n499(s, out),
		['р', s @ ..] => super::t8::n500(s, out),
		['у', s @ ..] => super::t8::n501(s, out),
		['ы', s @ ..] => super::t8::n502(s, out),
		['э', s @ ..] => super::t8::n503(s, out),
		['ю', s @ ..] => super::t8::n504(s, out),
		['я', s @ ..] => super::t8::n505(s, out),
		_ => false,
	}
}

/// Words ending -симгуащ.
pub(crate) fn n96617(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 95592);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 114211);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 126117);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -товарищ.
pub(crate) fn n96618(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 95593);
	s.is_empty()
}

/// Words ending -каращищ.
pub(crate) fn n96619(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 95594);
	s.is_empty()
}

/// Words ending -мщ.
pub(crate) fn n509(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 508);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4439);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 18392);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ощ.
pub(crate) fn n510(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 509);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 4440);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 18393);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 4441);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 18394);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 18395);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рщ.
pub(crate) fn n511(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 510);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 4442);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 18396);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ущ.
pub(crate) fn n512(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 511);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 4443);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 18397);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 42268);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 70428);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 95595);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 42269);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 70429);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 95596);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 114212);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыщ.
pub(crate) fn n513(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 512);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 4444);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18398);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 42270);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 70430);
									s.is_empty()
								}
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
			super::push(out, 4445);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 18399);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 4446);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 18400);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 42271);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 70431);
									s.is_empty()
								}
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
			super::push(out, 4447);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 18401);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 42272);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 18402);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ющ.
pub(crate) fn n514(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 513);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 4448);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 18403);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ящ.
pub(crate) fn n515(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 514);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 4449);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 18404);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ы.
pub(crate) fn n27(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 515);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4450);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 18405);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 42273);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 70432);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 95597);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 4451);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 18406);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 42274);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 70433);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 95598);
													match s {
														[] => true,
														['п', s @ ..] => super::t8::n115425(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 4452);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 18407);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 42275);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 70434);
											s.is_empty()
										}
										_ => false,
									}
								}
								['г', s @ ..] => {
									super::push(out, 42276);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 70435);
											s.is_empty()
										}
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
					super::push(out, 4453);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 18408);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 42277);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 4454);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 18409);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 516);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4455);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 4456);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 18410);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 42278);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 70436);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 95599);
													match s {
														[] => true,
														['з', s @ ..] => super::t8::n115426(s, out),
														_ => false,
													}
												}
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
							super::push(out, 18411);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 42279);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 18412);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 42280);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 70437);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 95600);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 4457);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 18413);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 42281);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 70438);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 95601);
													s.is_empty()
												}
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
							super::push(out, 18414);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 4458);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 18415);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 42282);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 70439);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 95602);
													match s {
														[] => true,
														['л', s @ ..] => super::t8::n115427(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 4459);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 18416);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 42283);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 4460);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 18417);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 18418);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 42284);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 70440);
											match s {
												[] => true,
												['о', s @ ..] => super::t8::n96628(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ч', s @ ..] => super::t8::n42706(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => super::t8::n4472(s, out),
				['ч', s @ ..] => super::t8::n4473(s, out),
				_ => false,
			}
		}
		['д', s @ ..] => super::t8::n518(s, out),
		['з', s @ ..] => super::t8::n519(s, out),
		['к', s @ ..] => super::t8::n520(s, out),
		['л', s @ ..] => super::t8::n521(s, out),
		['м', s @ ..] => super::t8::n522(s, out),
		['н', s @ ..] => super::t8::n523(s, out),
		['п', s @ ..] => super::t8::n524(s, out),
		['р', s @ ..] => super::t8::n525(s, out),
		['с', s @ ..] => super::t8::n526(s, out),
		['т', s @ ..] => super::t8::n527(s, out),
		['ф', s @ ..] => super::t8::n528(s, out),
		['ц', s @ ..] => super::t8::n529(s, out),
		['ш', s @ ..] => super::t8::n530(s, out),
		_ => false,
	}
}

/// Words ending -ь.
pub(crate) fn n28(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 530);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4578);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 18676);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 42573);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 70660);
											s.is_empty()
										}
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
					super::push(out, 4579);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 18677);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 42574);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 4580);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 18678);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 42575);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 70661);
											s.is_empty()
										}
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
					super::push(out, 4581);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18679);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 18680);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 42576);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 70662);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 95744);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', '\u{1}', s @ ..] => {
							super::push(out, 18681);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 18682);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 42577);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 18683);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 42578);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 4582);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 18684);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 42579);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 70663);
											s.is_empty()
										}
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
					super::push(out, 4583);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 18685);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 42580);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 42581);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 70664);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 70665);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 95745);
													s.is_empty()
												}
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
							super::push(out, 18686);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 42582);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 70666);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 70667);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 95746);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 4584);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 18687);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 18688);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 42583);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 4585);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 18689);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 18690);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 42584);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 18691);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => super::t8::n532(s, out),
		['д', s @ ..] => super::t8::n533(s, out),
		['ж', s @ ..] => {
			super::push(out, 533);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4608);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 18754);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 42669);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 4609);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 18755);
							match s {
								[] => true,
								['о', s @ ..] => super::t8::n43092(s, out),
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 18756);
							match s {
								[] => true,
								['а', s @ ..] => super::t8::n43093(s, out),
								_ => false,
							}
						}
						['р', s @ ..] => super::t8::n18862(s, out),
						['т', s @ ..] => super::t8::n18863(s, out),
						_ => false,
					}
				}
				['о', s @ ..] => super::t8::n4621(s, out),
				['ы', s @ ..] => super::t8::n4622(s, out),
				['я', s @ ..] => super::t8::n4623(s, out),
				_ => false,
			}
		}
		['з', s @ ..] => super::t8::n535(s, out),
		['л', s @ ..] => super::t8::n536(s, out),
		['м', s @ ..] => super::t9::n537(s, out),
		['н', s @ ..] => super::t9::n538(s, out),
		['п', s @ ..] => super::t9::n539(s, out),
		['р', s @ ..] => super::t9::n540(s, out),
		['с', s @ ..] => super::t9::n541(s, out),
		['т', s @ ..] => super::t9::n542(s, out),
		['ф', s @ ..] => super::t9::n543(s, out),
		['ч', s @ ..] => super::t9::n544(s, out),
		['ш', s @ ..] => super::t9::n545(s, out),
		['щ', s @ ..] => super::t9::n546(s, out),
		_ => false,
	}
}

/// Words ending -э.
pub(crate) fn n29(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 546);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4716);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 19249);
							s.is_empty()
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 4717);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 19250);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 4718);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 19251);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 547);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4719);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 19252);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 548);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 4720);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 19253);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 549);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 4721);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 19254);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 4722);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 19255);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 43806);
									s.is_empty()
								}
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
			super::push(out, 550);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 4723);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 19256);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 43807);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 72199);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 551);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 4724);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 552);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 4725);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 19257);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 43808);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 72200);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 97227);
													match s {
														[] => true,
														['д', s @ ..] => super::t9::n116895(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 4726);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 19258);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 4727);
					match s {
						[] => true,
						['э', s @ ..] => {
							super::push(out, 19259);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 43809);
									s.is_empty()
								}
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
			super::push(out, 553);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 4728);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 19260);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 554);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 4729);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 19261);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 43810);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 4730);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 19262);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 555);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4731);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 19263);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 43811);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 72201);
											s.is_empty()
										}
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
					super::push(out, 4732);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 19264);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 43812);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 4733);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 19265);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 43813);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 72202);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 19266);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 43814);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 72203);
											s.is_empty()
										}
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
					super::push(out, 4734);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 19267);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 43815);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 4735);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 19268);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 4736);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 19269);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 43816);
									s.is_empty()
								}
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
			super::push(out, 556);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 4737);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 19270);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 4738);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 19271);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 43817);
									s.is_empty()
								}
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
			super::push(out, 557);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4739);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 19272);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 43818);
									s.is_empty()
								}
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
			super::push(out, 558);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 4740);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 19273);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 43819);
									s.is_empty()
								}
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
			super::push(out, 559);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 4741);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 19274);
							match s {
								[] => true,
								['в', s @ ..] => super::t9::n44244(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => super::t9::n4753(s, out),
				_ => false,
			}
		}
		['у', s @ ..] => super::t9::n561(s, out),
		['х', s @ ..] => super::t9::n562(s, out),
		['ю', s @ ..] => {
			super::push(out, 562);
			s.is_empty()
		}
		['я', s @ ..] => super::t9::n564(s, out),
		_ => false,
	}
}

/// Words ending -ю.
pub(crate) fn n30(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 564);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4748);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 4749);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19281);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 565);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 4750);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 19282);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 43824);
									s.is_empty()
								}
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
			super::push(out, 566);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4751);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 19283);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 4752);
					match s {
						[] => true,
						['к', 'с', 'е', 'р', 'п', '\u{1}', s @ ..] => {
							super::push(out, 19284);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 19285);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 43825);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 72208);
											match s {
												[] => true,
												['й', s @ ..] => {
													super::push(out, 97231);
													match s {
														[] => true,
														['а', s @ ..] => super::t9::n116899(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 567);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 4753);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 19286);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 43826);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 4754);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 19287);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 43827);
									s.is_empty()
								}
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
			super::push(out, 568);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 4755);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 19288);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 43828);
									s.is_empty()
								}
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
			super::push(out, 569);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 4756);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 19289);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 43829);
									s.is_empty()
								}
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
			super::push(out, 570);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 4757);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 19290);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 43830);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 72209);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 19291);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 43831);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 72210);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 571);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 4758);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 19292);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 43832);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 72211);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 572);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 4759);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 19293);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 43833);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 4760);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 573);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4761);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 19294);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 43834);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 72212);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 97232);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 4762);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 19295);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 43835);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 72213);
											s.is_empty()
										}
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
					super::push(out, 4763);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 19296);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 4764);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 19297);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 574);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4765);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 19298);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 4766);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 19299);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 43836);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 43837);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 72214);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 97233);
													s.is_empty()
												}
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
							super::push(out, 19300);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 43838);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 72215);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 19301);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 575);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 4767);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 19302);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 43839);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 4768);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 19303);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => super::t9::n577(s, out),
		['т', s @ ..] => {
			super::push(out, 577);
			match s {
				[] => true,
				['ю', s @ ..] => super::t9::n4783(s, out),
				_ => false,
			}
		}
		['ц', s @ ..] => super::t9::n579(s, out),
		['ч', s @ ..] => super::t9::n580(s, out),
		['ш', s @ ..] => super::t9::n581(s, out),
		['ь', s @ ..] => super::t9::n582(s, out),
		_ => false,
	}
}

/// Words ending -я.
pub(crate) fn n31(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31);
	match s {
		[] => true,
		['а', s @ ..] => super::t9::n583(s, out),
		['д', s @ ..] => {
			super::push(out, 583);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4801);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 19403);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 19404);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 4802);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 19405);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 4803);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 19406);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 44027);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 72483);
											s.is_empty()
										}
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
					super::push(out, 4804);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 19407);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 44028);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 72484);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 19408);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 19409);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 44029);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 4805);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 19410);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t9::n585(s, out),
		['з', s @ ..] => {
			super::push(out, 585);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 4824);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 19474);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 44109);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 4825);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 19475);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 44110);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 72550);
											s.is_empty()
										}
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
					super::push(out, 4826);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 19476);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 44111);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 4827);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 19477);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 44112);
									s.is_empty()
								}
								_ => false,
							}
						}
						['э', s @ ..] => {
							super::push(out, 19478);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 4828);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 19479);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 44113);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 4829);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 19480);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 44114);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 72551);
											s.is_empty()
										}
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
					super::push(out, 4830);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 19481);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t9::n587(s, out),
		['й', s @ ..] => {
			super::push(out, 587);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4851);
					match s {
						[] => true,
						['б', s @ ..] => super::t9::n19757(s, out),
						['в', s @ ..] => super::t9::n19758(s, out),
						['к', s @ ..] => super::t9::n19759(s, out),
						['л', s @ ..] => super::t9::n19760(s, out),
						['м', s @ ..] => super::t9::n19761(s, out),
						['п', s @ ..] => super::t9::n19762(s, out),
						['р', s @ ..] => super::t9::n19763(s, out),
						['с', s @ ..] => super::t9::n19764(s, out),
						['т', s @ ..] => super::t9::n19765(s, out),
						['х', s @ ..] => super::t9::n19766(s, out),
						_ => false,
					}
				}
				['о', s @ ..] => super::t9::n4863(s, out),
				['у', s @ ..] => super::t9::n4864(s, out),
				_ => false,
			}
		}
		['л', s @ ..] => super::t9::n589(s, out),
		['м', s @ ..] => super::t9::n590(s, out),
		['н', s @ ..] => super::t9::n591(s, out),
		['о', s @ ..] => super::t9::n592(s, out),
		['р', s @ ..] => super::t9::n593(s, out),
		['с', s @ ..] => super::t9::n594(s, out),
		['т', s @ ..] => super::t9::n595(s, out),
		['у', s @ ..] => super::t9::n596(s, out),
		['ы', s @ ..] => super::t9::n597(s, out),
		['ь', s @ ..] => super::t9::n598(s, out),
		['я', s @ ..] => super::t9::n599(s, out),
		_ => false,
	}
}

