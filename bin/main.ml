

let compile (program: string) : string =
  String.concat "\n"
  [ ".global _entry";
    "_entry:";
    Printf.sprintf "\tmov x0, %s" program;
    "\tret"]

let compile_to_file (program : string) : unit =
  let file = open_out "program.s" in 
  output_string file (compile program);
  close_out file


let compile_and_run (program : string) : string =
  match program with 
  "" -> "" 
  | _ ->
    compile_to_file program;
    ignore (Unix.system "as program.s -o program.o");
    ignore (Unix.system "gcc program.o runtime.o -o program");
    let inp = Unix.open_process_in "./program" in
    let r = input_line inp in
    close_in inp;  
    r
;;



let _ = compile_and_run ""