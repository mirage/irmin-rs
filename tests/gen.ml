let save filename t v =
  let s = Repr.(unstage (to_bin_string t)) v in
  let cwd = Sys.getcwd () in
  let name = Filename.basename cwd in
  let prefix = if name = "tests" then  "" else "tests/" in
  Lwt_io.chars_to_file (prefix ^ filename ^ ".bin") (Lwt_stream.of_string s)

let int_string_pair () =
  save "int_string_pair" Repr.(pair int string) (123, "abc")

let int_long_string_pair () =
  save "int_long_string_pair" Repr.(pair int string) (500, String.make 4096 'A')

type struct1 = {a: int; b: string array} [@@deriving repr]
let struct1 () =
  save "struct1" struct1_t {a = 999; b = Array.make 16 "B"}

type enum1 = A of float | B of string option [@@deriving repr]
let enum1 () =
  save "enum1" Repr.(pair enum1_t enum1_t) (A 4.5, B None)

let tests = [
  int_string_pair;
  int_long_string_pair;
  struct1;
  enum1;
]


let () =
  Lwt_main.run (
    Lwt_list.iter_s (fun x -> x ()) tests)

