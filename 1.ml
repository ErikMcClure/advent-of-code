open import "prelude.ml"

let rec read_lines file : list string = match Io.read_line file with
  | Some s -> s :: (read_lines file)
  | None -> []

let input = match (sequence @@ map parse_int (read_lines @@ Io.open_for_reading "1.txt")) with
  | Some s -> s
  | None -> []

let rec fold2 xs f a = match xs with 
  | Nil -> a
  | Cons (head, tail) -> (f head tail (fold2 tail f a))

let check x xs a = match xs with
  | Nil -> a
  | Cons (y, _) -> if x < y then 1 + a else a

(* Part 1 *)
(* let increases = (fold2 input check 0) *)

(* Part 2 *)
let winadd x y z = x + y + z

(* let regions = input |> map (fun p -> let ps = scanl (+) 0 p in zip_with (-) (tail @@ tail @@ tail ps) ps) *)
let merge = (zip_with (,) (zip_with (,) input (tail input)) (tail (tail input)))
let diff = (map (fun ((x,y),z) -> x+y+z) merge)

(* let windows = (map2 winadd input) *)

let increases = (fold2 diff check 0)

let () = print(increases)