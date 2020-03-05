/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   mvg.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: lfalkau <lfalkau@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2020/02/27 22:37:02 by lfalkau           #+#    #+#             */
/*   Updated: 2020/03/05 02:14:03 by lfalkau          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::io;
use colored::*;
use crate::mvp;
use crate::board;

#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
pub struct Move
{
	pub from: board::Box,
	pub to: board::Box,
}

pub fn is_legal_move(player: board::Color, mv: Move, board: &board::Board) -> bool {
	let legal_moves = get_legal_moves_for(player, board);
	for legal_move in legal_moves.iter() {
		if mv == *legal_move {
			return true;
		}
	}
	return false;
}

fn get_legal_moves_for(player: board::Color, board: &board::Board) -> Vec<Move> {
	
}

pub fn is_owned_by(player: board::Color, bx: board::Box, board: &board::Board) -> bool
{
	return board.color_at(bx.x, bx.y) == player;
}

fn chess_for(player: board::Color, b: &mut board::Board) -> bool
{
	let mut king_pos;

	if player == board::Color::White
	{
		king_pos = b.get_pos('k');
		for y in 0..8
		{
			for x in 0..8
			{
				let c = b.at(x, y);
				if c.is_uppercase()
				{
					let chess: bool;
					let m = mvg::Move {from: board::Box {x: x, y: y}, to: board::Box {x: king_pos.x, y: king_pos.y}};
					chess = match c
					{
						'P' => mvp::move_pawn(board::Color::Black, &m, b),
						'R' => mvp::move_rock(&m, b),
						'H' => mvp::move_knight(&m),
						'B' => mvp::move_bishop(&m, b),
						'Q' => mvp::move_queen(&m, b),
						'K' => mvp::move_king(&m),
						_ => false
					};
					if chess { return true; }
				}
			}
		}
	}
	if player == board::Color::Black
	{
		king_pos = b.get_pos('K');
		for y in 0..8
		{
			for x in 0..8
			{
				let c = b.at(x, y);
				if c.is_lowercase()
				{
					let chess: bool;
					let m = mvg::Move {from: board::Box {x: x, y: y}, to: board::Box {x: king_pos.x, y: king_pos.y}};
					chess = match c
					{
						'p' => mvp::move_pawn(board::Color::White, &m, b),
						'r' => mvp::move_rock(&m, b),
						'h' => mvp::move_knight(&m),
						'b' => mvp::move_bishop(&m, b),
						'q' => mvp::move_queen(&m, b),
						'k' => mvp::move_king(&m),
						_ => false
					};
					if chess { return true; }
				}
			}
		}
	}
	return false;
}

pub fn try_proceed(m: &Move, board: &mut board::Board) -> bool
{
	let src_color = board.color_at(m.from.x, m.from.y);
	let dst_color = board.color_at(m.to.x, m.to.y);

	if src_color == dst_color
	{
		println!("{}", format!("{}", "Suicide is unauthaurized.".bright_red()));
		return false;
	}
	let success = match board.at(m.from.x, m.from.y)
	{
		'P' | 'p' => mvp::move_pawn(src_color, m, board),
		'R' | 'r' => mvp::move_rock(m, board),
		'H' | 'h' => mvp::move_knight(m),
		'B' | 'b' => mvp::move_bishop(m, board),
		'Q' | 'q' => mvp::move_queen(m, board),
		'K' | 'k' => mvp::move_king(m),
		_ => panic!("Impossible statement")
	};
	if !success
	{
		println!("{}", format!("{}", "You can't do that.".bright_red()));
	}
	return success;
}
