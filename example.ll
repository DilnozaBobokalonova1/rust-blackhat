; Example LLVM IR file: example.ll

define i32 @addition(i32 %a, i32 %b) {
  ; Entry block, basic block label: entry
  entry:
    ; Add the two parameters
    %result = add i32 %a, %b
    
    ; Return the result
    ret i32 %result
}

