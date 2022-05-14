use freertos_cargo_build::*;
fn main() {
    let mut builder = Builder::new();

    // Path to FreeRTOS kernel or set ENV "FREERTOS_SRC" instead
    let kernel_location : String = String::from("FreeRTOS/Source");
    let allocator_choice: String = String::from("heap4.c");
    let compiler_choice : String = String::from("MSVC-MingW");
    let kernel_config : String = String::from("src");

    builder.freertos(kernel_location);
    builder.freertos_config(kernel_config);       // Location of `FreeRTOSConfig.h`
    builder.freertos_port(compiler_choice); // Port dir relativ to 'FreeRTOS-Kernel/portable'
    //builder.heap(allocator_choice);

    builder.get_cc().file("src/hooks.c");
    builder.get_cc().file("src/Run-time-stats-utils.c");

    // Set the heap_?.c allocator to use from
    // 'FreeRTOS-Kernel/portable/MemMang' (Default: heap_4.c)

    // b.get_cc().file("More.c");   // Optional additional C-Code to be compiled
    //C:\\Users\\maxim\\CLionProjects\\FreeRTOS-Kernel\\portable\\MemMang\\

    builder.compile().unwrap_or_else(|e| { panic!("{}", e.to_string()) });
}