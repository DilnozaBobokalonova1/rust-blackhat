; ModuleID = 'example.bc'
source_filename = "example.ll"

define i32 @addition(i32 %a, i32 %b) {
entry:
  %result = add i32 %a, %b
  ret i32 %result
}
