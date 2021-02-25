(*open Lwt.Syntax*)
open Lwt.Infix

module Pack_config = struct
  let stable_hash = 256

  let entries = 32
end

module Store = Irmin_pack.KV (Pack_config) (Irmin.Contents.String)

type contents = String | Json | Json_value

let contents : contents -> (module Irmin.Contents.S) = function
  | String -> (module Irmin.Contents.String)
  | Json -> (module Irmin.Contents.Json)
  | Json_value -> (module Irmin.Contents.Json_value)

module OCaml = struct
  let config root = Irmin_pack.config root

  let repo config : Store.repo Lwt.t = Store.Repo.v config

  module Contents = struct end

  module Store = struct
    let master (repo : Store.repo Lwt.t) = repo >>= Store.master

    let find store key =
      let key = Irmin.Type.of_string Store.key_t key |> Result.get_ok in
      Lwt_main.run
        ( store >>= fun store ->
          Store.find store key >>= function
          | Some x -> Lwt.return_some (Irmin.Type.to_string Store.contents_t x)
          | None -> Lwt.return_none )

    let set store key value message =
      let key = Irmin.Type.of_string Store.key_t key |> Result.get_ok in
      let info = Irmin_unix.info "%s" message in
      Lwt_main.run (store >>= fun store -> Store.set_exn store key ~info value)

    let mem store key =
      let key = Irmin.Type.of_string Store.key_t key |> Result.get_ok in
      Lwt_main.run (store >>= fun store -> Store.mem store key)

    let remove store key message =
      let info = Irmin_unix.info "%s" message in
      let key = Irmin.Type.of_string Store.key_t key |> Result.get_ok in
      Lwt_main.run (store >>= fun store -> Store.remove store key ~info)
  end

  type f = Function : 'a -> f

  let functions =
    [
      ("config", Function config);
      ("repo", Function repo);
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
