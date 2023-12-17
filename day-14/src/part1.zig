const std = @import("std");

const Coord = struct {
    x: usize,
    y: usize,
};

const Slide = struct {
    start_row: usize = 0,
    num_round_rocks: usize = 0,
};

fn process(allocator: std.mem.Allocator, input: []const u8) !usize {
    var map = std.AutoHashMap(usize, Slide).init(allocator);
    defer map.deinit();

    var all_things = std.ArrayList(Slide).init(allocator);
    defer all_things.deinit();

    var column: usize = 0;
    var row: usize = 0;

    for (input) |char| {
        if (row == 0) {
            _ = try map.put(column, .{});
        }
        switch (char) {
            'O' => {
                var slide = map.getPtr(column);
                if (slide != null) {
                    slide.?.num_round_rocks += 1;
                }
            },
            '#' => {
                const kv = map.fetchRemove(column);
                if (kv != null) {
                    // std.debug.print("{d}:{d}, start_row {d}: {d} rocks\n", .{ row, column, kv.?.value.start_row, kv.?.value.num_round_rocks });
                    try all_things.append(kv.?.value);
                }
                // +1 to indicate that rolling stones should stop at the row above
                try map.put(column, .{ .start_row = row + 1 });
            },
            '\n' => {
                row += 1;
                column = 0;
                continue;
            },
            else => {},
        }
        // for (all_things.items) |obj| {
        //     std.debug.print("{d}", .{obj.start_row});
        // }
        // std.debug.print("\n", .{});
        column += 1;
    }

    var iter = map.iterator();
    var v = iter.next();
    while (v != null) {
        try all_things.append(v.?.value_ptr.*);

        v = iter.next();
    }

    // std.debug.print("Rows: {d}\n", .{row});

    var total_points: usize = 0;
    for (all_things.items) |slide| {
        if (slide.num_round_rocks > 0) {
            for (slide.start_row..slide.start_row + slide.num_round_rocks) |i| {
                std.debug.print("COUNT ROW: {d}: {d}\n", .{ row - i + 1, i });
                total_points += row - i + 1;
            }
        }
    }

    return total_points;
}

pub fn main() !void {
    // Prints to stderr (it's a shortcut based on `std.io.getStdErr()`)
    std.debug.print("All your {s} are belong to us.\n", .{"codebase"});

    // stdout is for the actual output of your application, for example if you
    // are implementing gzip, then only the compressed bytes should be sent to
    // stdout, not any debugging messages.
    // const stdout_file = std.io.getStdOut().writer();
    // var bw = std.io.bufferedWriter(stdout_file);
    // const stdout = bw.writer();

    // try stdout.print("Run `zig build test` to run the tests.\n", .{});

    // try bw.flush(); // don't forget to flush!

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer _ = gpa.deinit();
    const input = @embedFile("input.txt");
    const out = try process(allocator, input);
    std.debug.print("OUT: {d}", .{out});
}

test "example" {
    const input =
        \\O....#....
        \\O.OO#....#
        \\.....##...
        \\OO.#O....O
        \\.O.....O#.
        \\O.#..O.#.#
        \\..O..#O..O
        \\.......O..
        \\#....###..
        \\#OO..#....
    ;
    const out = try process(std.testing.allocator, input);
    try std.testing.expectEqual(out, 136);
}
