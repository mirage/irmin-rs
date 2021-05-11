open Lwt.Syntax
open Lwt.Infix

module Pack_config = struct
  let stable_hash = 256

  let entries = 32
end

module Maker =
  Irmin_pack.KV
    (struct
      let version = `V1
    end)
    (Pack_config)

module Store = Maker.Make (Irmin.Contents.String)

type contents = String | Json | Json_value

let contents : contents -> (module Irmin.Contents.S) = function
  | String -> (module Irmin.Contents.String)
  | Json -> (module Irmin.Contents.Json)
  | Json_value -> (module Irmin.Contents.Json_value)

module OCaml = struct
  let config root = Irmin_pack.config root

  let repo config : Store.repo Lwt.t = Store.Repo.v config

  module Contents = struct end

  module Info = Irmin_unix.Info (Store.Info)

  let key_arg = Irmin.Type.(of_bin_string Store.key_t |> unstage)

  module Tree = struct
    let encode = Irmin.Type.(unstage (to_bin_string Store.Tree.concrete_t))

    let decode = Irmin.Type.(unstage (of_bin_string Store.Tree.concrete_t))

    let to_concrete t =
      Lwt_main.run
        (let+ c = Store.Tree.to_concrete t in
         encode c)

    let of_concrete t =
      let t = decode t |> Result.get_ok in
      Store.Tree.of_concrete t

    let add t k value =
      let key = key_arg k |> Result.get_ok in
      Lwt_main.run (Store.Tree.add t key value)

    let mem t k =
      let key = key_arg k |> Result.get_ok in
      Lwt_main.run (Store.Tree.mem t key)

    let empty () = Store.Tree.empty
  end

  module Store = struct
    let master (repo : Store.repo Lwt.t) = repo >>= Store.master

    let find store key =
      let key = key_arg key |> Result.get_ok in
      Lwt_main.run
        ( store >>= fun store ->
          Store.find store key >>= function
          | Some x -> Lwt.return_some x
          | None -> Lwt.return_none )

    let set store key value message =
      let key = key_arg key |> Result.get_ok in
      let info = Info.v "%s" message in
      Lwt_main.run (store >>= fun store -> Store.set_exn store key ~info value)

    let mem store key =
      let key = key_arg key |> Result.get_ok in
      Lwt_main.run (store >>= fun store -> Store.mem store key)

    let remove store key message =
      let info = Info.v "%s" message in
      let key = key_arg key |> Result.get_ok in
      Lwt_main.run (store >>= fun store -> Store.remove store key ~info)
  end

  type f = Function : 'a -> f

  let functions =
    [
      ("config", Function config);
      ("repo", Function repo);
      ("tree_of_concrete", Function Tree.of_concrete);
      ("tree_to_concrete", Function Tree.to_concrete);
      ("tree_add", Function Tree.add);
      ("tree_mem", Function Tree.mem);
      ("tree_empty", Function Tree.empty);
      ("store_master", Function Store.master);
      ("store_find", Function Store.find);
      ("store_set", Function Store.set);
      ("store_mem", Function Store.mem);
      ("store_remove", Function Store.remove);
    ]
end

let () =
  List.iter
    (fun (name, OCaml.Function f) -> Callback.register name f)
    OCaml.functions
