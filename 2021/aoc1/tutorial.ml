open import "prelude.ml"

let greet = let name = "world" in "hello" ^ name

let () = print greet

type mylist 'a = MyNil | MyCons of 'a * mylist 'a

let x = MyCons(1, MyCons(2, MyNil))

let rec mymap f xs = match xs with
  | MyNil -> MyNil
  | MyCons (head, tail) -> MyCons (f head, mymap f tail)

let y = mymap ( *2) x

let rec myfoldr xs f a = match xs with 
  | MyNil -> a
  | MyCons (head, tail) -> (f head (myfoldr tail f a))

let () = print (myfoldr x ( +) 0)

instance show 'a => show (mylist 'a) begin
  let show x = match x with
    | MyNil -> "nil"
    | MyCons (h, t) -> "[" ^ show h ^ ", " ^ show t ^ "]"
end

let () = print x

type nat = Z | S of nat

type vect ('len: nat) 'a =
  | Nil: vect Z 'a
  | Cons: 'a * vect 'len 'a -> vect (S 'len) 'a

let rec vmap (f: 'a -> 'b) (xs: vect 'len 'a) : vect 'len 'b = match xs with
  | Nil -> Nil
  | Cons (head, tail) -> Cons (f head, vmap f tail)

