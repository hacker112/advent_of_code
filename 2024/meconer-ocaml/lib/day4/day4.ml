open Core 

(* let filename = "lib/day4/example.txt" *)
let filename = "lib/day4/input.txt"
let day4Input = In_channel.read_lines filename

let width = String.length (List.hd_exn day4Input)
let height = List.length day4Input

type pos = {row : int; col : int}
let word = "XMAS"
let wl = String.length word

let list_to_array lst =
  Array.of_list (List.map ~f:String.to_array lst)

let letterArray = list_to_array day4Input

type directions = 
  | North 
  | NorthEast 
  | East 
  | SouthEast
  | South
  | SouthWest
  | West 
  | NorthWest

let allDirections = [North; NorthEast; East; SouthEast; South; SouthWest; West; NorthWest]

let moveDir dir p = 
    match dir with 
    | North -> {p with row = p.row-1}
    | NorthEast -> {row=p.row-1; col= p.col+1}
    | East -> {p with col = p.col+1}
    | SouthEast -> {row = p.row+1 ; col = p.col+1}
    | South -> {p with row = p.row+1}
    | SouthWest -> {row = p.row+1 ; col = p.col-1}
    | West -> {p with col = p.col-1}
    | NorthWest -> {row = p.row-1 ; col = p.col-1}

let getPosList (p:pos) (dir:directions) : pos list = 
  let rec inner count p = 
    match count with 
    | 0 -> []
    | _ -> p::inner (count-1) (moveDir dir p)
  in
  let lst = inner wl p in
  if List.length lst < wl then [] else lst
  
let getLetterAt (p:pos)  =  
  if p.row >=0 && p.row <width && p.col >= 0 && p.col < height then
    (Array.get (Array.get letterArray p.row) p.col)
  else 
    ' '

let getWordFromPoint (p:pos) (dir:directions)= 
  let letterPositions = getPosList p dir in

  let rec inner (accum:char list) (posList:pos list) = 
    match posList with
    | [] -> accum
    | p::tl -> inner ((getLetterAt p)::accum) tl
  in
  List.rev (inner [] letterPositions)
    

let getWordsFromPoint p = 
  List.map ~f:(fun dir -> getWordFromPoint p dir) allDirections

let isCorrectWord (lst:char list) = 
  match lst with 
  | 'X'::'M'::'A'::'S'::[] -> 1
  | _ -> 0

let countFromPoint p = 
  let words = getWordsFromPoint p in
  let corrects = List.map ~f:(isCorrectWord) words in
  List.fold ~init:0 ~f:(+) corrects

let countRow row = 
  let rec inner accum col =
    match col with 
    | c when c=width -> accum
    | c -> inner (accum + countFromPoint {row;col=c}) (col+1)
  in
  inner 0 0

let countAll =
  let rec inner accum row =
    match row with 
    | r when r=height -> accum
    | r -> inner (accum + countRow r) (r+1)
  in
  inner 0 0 

let resultP1 = countAll

(*
Correct combos appear when the letter at a point is A and
moving NW, NE, SW and SE has two each of the letters M and S
*)

let getXLetters p = 
  let allDirections = [NorthWest; NorthEast; SouthWest; SouthEast] in
  let posList = List.map ~f:(fun dir -> moveDir dir p) allDirections in
  List.map ~f:(getLetterAt) posList 

let correctCombos = ["MMSS"; "MSMS"; "SMSM"; "SSMM"]

let isCorrectP2 p = 
  if Char.(=) (getLetterAt p) 'A' then 
      let xLetters = getXLetters p in
      let str = String.of_char_list xLetters in
      List.exists ~f:(fun s -> String.equal str s) correctCombos
  else false


let countRowP2 row = 
  let rec inner accum col =
    match col with 
    | c when c=width -> accum
    | c -> if isCorrectP2 {row; col=c} then 
              inner (accum + 1) (col+1)
           else inner accum (col+1)
  in
  inner 0 0

let countAllP2 =
  let rec inner accum row =
    match row with 
    | r when r=height -> accum
    | r -> inner (accum + countRowP2 r) (r+1)
  in
  inner 0 0 


let resultP2 = countAllP2

