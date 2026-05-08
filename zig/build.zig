const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const exe_module = createIxModule(b, target, optimize);
    const exe = b.addExecutable(.{
        .name = "ix-zig",
        .root_module = exe_module,
    });

    b.installArtifact(exe);

    const run_cmd = b.addRunArtifact(exe);
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    const run_step = b.step("run", "Run ix-zig");
    run_step.dependOn(&run_cmd.step);

    const unit_tests = b.addTest(.{
        .root_module = createIxModule(b, target, optimize),
    });

    const test_cmd = b.addRunArtifact(unit_tests);
    const test_step = b.step("test", "Run Zig unit tests");
    test_step.dependOn(&test_cmd.step);
}

fn createIxModule(
    b: *std.Build,
    target: std.Build.ResolvedTarget,
    optimize: std.builtin.OptimizeMode,
) *std.Build.Module {
    const root_module = b.createModule(.{
        .root_source_file = b.path("src/main.zig"),
        .target = target,
        .optimize = optimize,
        // link_libc is required because StringZilla's AVX2 headers pull in
        // immintrin.h -> xmmintrin.h -> mm_malloc.h -> stdlib.h.
        .link_libc = true,
    });

    // StringZilla SIMD search kernels are compiled from a C shim that wraps
    // the header-only library into linkable symbols for Zig's extern fn FFI.
    root_module.addCSourceFile(.{
        .file = b.path("src/sz_shim.c"),
        .flags = &.{
            "-mavx2",
            "-O3",
            "-DNDEBUG",
            "-std=c11",
        },
    });
    root_module.addIncludePath(b.path("../.refs/stringzilla/include"));

    return root_module;
}
