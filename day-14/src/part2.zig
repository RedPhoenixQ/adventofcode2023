const std = @import("std");

const Tile = enum {
    Square,
    Round,
    Empty,
};

const Direction = enum {
    North,
    West,
    South,
    East,
};

const Grid = struct {
    rows: usize = 1,
    columns: usize = 0,
    array: std.ArrayList(Tile),

    fn deinit(self: Grid) void {
        self.array.deinit();
    }

    fn from_string(input: []const u8, allocator: std.mem.Allocator) !Grid {
        var grid = Grid{ .array = undefined };
        grid.array = std.ArrayList(Tile).init(allocator);

        for (input) |char| {
            grid.columns += 1;
            switch (char) {
                'O' => {
                    try grid.array.append(Tile.Round);
                },
                '#' => {
                    try grid.array.append(Tile.Square);
                },
                '.' => {
                    try grid.array.append(Tile.Empty);
                },
                '\n' => {
                    std.debug.print("NEWLINE {d} {d}\n", .{ grid.columns, grid.rows });
                    grid.rows += 1;
                    grid.columns = 0;
                },
                else => {},
            }
        }

        std.debug.print("{d} items, cols {d}, rows {d}\n", .{ grid.array.items.len, grid.columns, grid.rows });
        return grid;
    }

    fn iterator(self: *Grid) GridIterator {
        return .{
            .items = self.array.items,
            .width = self.columns - 1,
            .height = self.rows - 1,
        };
    }
};

const GridContext = struct {
    pub fn hash(self: @This(), grid: Grid) u64 {
        _ = self;
        var gpa = std.heap.GeneralPurposeAllocator(.{}){};
        const allocator = gpa.allocator();
        defer _ = gpa.deinit();
        var slice = std.ArrayList(u8).initCapacity(allocator, grid.array.items.len) catch unreachable;
        defer slice.deinit();
        var num: u8 = 0;
        for (grid.array.items) |tile| {
            slice.append(@intFromBool(tile == .Round)) catch unreachable;
            num = 0;
        }
        return std.hash.Fnv1a_64.hash(slice.items);
    }
    pub fn eql(self: @This(), a: Grid, b: Grid) bool {
        _ = self;
        return std.mem.eql(Tile, a.array.items, b.array.items);
    }
};

const GridIterator = struct {
    index: usize = 0,
    offset: usize = 0,
    items: []Tile,
    width: usize,
    height: usize,
    direction: Direction = .North,
    current_line: ?LineIterator = null,

    fn reset(self: *GridIterator) void {
        self.direction = .North;
        self.index = 0;
        self.current_line = null;
    }

    fn next(self: *GridIterator) ?*LineIterator {
        defer self.index += 1;
        switch (self.direction) {
            .North => {
                if (self.index > self.height) {
                    self.index = 0;
                    self.direction = .West;
                }
            },
            .West => {
                if (self.index > self.width) {
                    self.index = 0;
                    self.direction = .South;
                }
            },
            .South => {
                if (self.index > self.height) {
                    self.index = 0;
                    self.direction = .East;
                }
            },
            .East => {
                if (self.index > self.width) {
                    return null;
                }
            },
        }
        self.offset = self.index;
        // std.debug.print("\nGrid i: {d}", .{self.index});
        self.current_line = .{
            .grid = self,
        };
        return &self.current_line.?;
    }

    fn index_of(self: *GridIterator, x: usize, y: usize) usize {
        return x + y * (self.width + 1);
    }
};

const LineIterator = struct {
    index: usize = 0,
    grid: *GridIterator,

    fn next(self: *LineIterator) ?*Tile {
        defer self.index += 1;
        const pos = switch (self.grid.direction) {
            .North => pos: {
                if (self.index > self.grid.height) return null;
                break :pos self.grid.index_of(self.grid.offset, self.grid.height - self.index);
            },
            .West => pos: {
                if (self.index > self.grid.width) return null;
                break :pos self.grid.index_of(self.grid.width - self.index, self.grid.offset);
            },
            .South => pos: {
                if (self.index > self.grid.height) return null;
                break :pos self.grid.index_of(self.grid.offset, self.index);
            },
            .East => pos: {
                if (self.index > self.grid.width) return null;
                break :pos self.grid.index_of(self.index, self.grid.offset);
            },
        };
        // std.debug.print("\n{d}: {d} - ", .{ self.index, pos });
        return &self.grid.items[pos];
    }
};

const MAX_CYCLES = 1000000000;

fn process(allocator: std.mem.Allocator, input: []const u8) !usize {
    var grid = try Grid.from_string(input, allocator);
    defer grid.deinit();

    std.debug.print("{d} items, cols {d}, rows {d}\n", .{ grid.array.items.len, grid.columns, grid.rows });

    var grid_iter = grid.iterator();

    var permutations = std.ArrayList(std.ArrayList(Tile)).init(allocator);
    defer {
        for (permutations.items) |sub| {
            sub.deinit();
        }
        permutations.deinit();
    }

    var count_round_tiles: usize = 0;
    var valid_stack = std.ArrayList(*Tile).init(allocator);
    defer valid_stack.deinit();

    outer: for (0..MAX_CYCLES) |i| {
        std.debug.print("\rIter {d} ", .{i});

        while (grid_iter.next()) |line| {
            while (line.*.next()) |tile| {
                switch (tile.*) {
                    .Empty => {
                        try valid_stack.append(tile);
                    },
                    .Round => {
                        count_round_tiles += 1;
                        tile.* = .Empty;
                        try valid_stack.append(tile);
                    },
                    .Square => {
                        for (0..count_round_tiles) |_| {
                            const valid_tile = valid_stack.pop();
                            valid_tile.* = .Round;
                        }
                        count_round_tiles = 0;
                        valid_stack.clearRetainingCapacity();
                    },
                }
            }
            for (0..count_round_tiles) |_| {
                const valid_tile = valid_stack.pop();
                valid_tile.* = .Round;
            }
            count_round_tiles = 0;
            valid_stack.clearRetainingCapacity();
        }

        for (permutations.items, 0..) |perm, cycle_start| {
            if (std.mem.eql(Tile, grid.array.items, perm.items)) {
                const loop_len = i - cycle_start;
                const remaining_iterations = (MAX_CYCLES - cycle_start) % (loop_len) - 1;
                std.debug.print("Found matching permutation at {d}, loop_len = {d}, remaining = {d}\n", .{ i, loop_len, remaining_iterations });
                const ending_perm = permutations.orderedRemove(cycle_start + remaining_iterations);
                grid.array.deinit();
                grid.array = ending_perm;
                break :outer;
            }
        } else {
            std.debug.print("Adding permutation at {d}\n", .{i});
            try permutations.append(try grid.array.clone());
        }

        grid_iter.reset();
    }

    // for (grid.array.items, 0..) |tile, i| {
    //     if (i % grid.columns == 0) {
    //         std.debug.print("\n", .{});
    //     }
    //     const tile_char: u8 = switch (tile) {
    //         .Empty => '.',
    //         .Round => 'O',
    //         .Square => '#',
    //     };
    //     std.debug.print("{c}", .{tile_char});
    // }

    var total_points: usize = 0;
    for (0..grid.rows) |y| {
        for (0..grid.columns) |x| {
            const tile = grid.array.items[x + y * grid.columns];
            if (tile == .Round) {
                total_points += grid.rows - y;
            }
        }
    }

    return total_points;
}

fn string_to_grid(input: []const u8, allocator: std.mem.Allocator) std.ArrayList(Tile) {
    _ = allocator;
    _ = input;
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
    const out: usize = try process(std.testing.allocator, input);
    try std.testing.expectEqual(@as(usize, 64), out);
}

test "iterations" {
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
    var grid = try Grid.from_string(input, std.testing.allocator);
    defer grid.deinit();
    var grid_iter = grid.iterator();

    const cycle1 =
        \\.....#....
        \\....#...O#
        \\...OO##...
        \\.OO#......
        \\.....OOO#.
        \\.O#...O#.#
        \\....O#....
        \\......OOOO
        \\#...O###..
        \\#..OO#....
    ;
    const cycle1grid = try Grid.from_string(cycle1, std.testing.allocator);
    defer cycle1grid.deinit();
    const cycle2 =
        \\.....#....
        \\....#...O#
        \\.....##...
        \\..O#......
        \\.....OOO#.
        \\.O#...O#.#
        \\....O#...O
        \\.......OOO
        \\#..OO###..
        \\#.OOO#...O
    ;
    const cycle2grid = try Grid.from_string(cycle2, std.testing.allocator);
    defer cycle2grid.deinit();
    const cycle3 =
        \\.....#....
        \\....#...O#
        \\.....##...
        \\..O#......
        \\.....OOO#.
        \\.O#...O#.#
        \\....O#...O
        \\.......OOO
        \\#...O###.O
        \\#.OOO#...O
    ;
    const cycle3grid = try Grid.from_string(cycle3, std.testing.allocator);
    defer cycle3grid.deinit();

    var count_round_tiles: usize = 0;
    var valid_stack = std.ArrayList(*Tile).init(std.testing.allocator);
    defer valid_stack.deinit();

    for (0..3) |i| {
        std.debug.print("Iter {d}\n", .{i});

        while (grid_iter.next()) |line| {
            while (line.*.next()) |tile| {
                switch (tile.*) {
                    .Empty => {
                        try valid_stack.append(tile);
                    },
                    .Round => {
                        count_round_tiles += 1;
                        tile.* = .Empty;
                        try valid_stack.append(tile);
                    },
                    .Square => {
                        for (0..count_round_tiles) |_| {
                            const valid_tile = valid_stack.pop();
                            valid_tile.* = .Round;
                        }
                        count_round_tiles = 0;
                        valid_stack.clearRetainingCapacity();
                    },
                }
            }
            for (0..count_round_tiles) |_| {
                const valid_tile = valid_stack.pop();
                valid_tile.* = .Round;
            }
            count_round_tiles = 0;
            valid_stack.clearRetainingCapacity();

            for (grid.array.items, 0..) |tile, idx| {
                if (idx % grid.columns == 0) {
                    std.debug.print("\n", .{});
                }
                const tile_char: u8 = switch (tile) {
                    .Empty => '.',
                    .Round => 'O',
                    .Square => '#',
                };
                std.debug.print("{c}", .{tile_char});
            }
            std.debug.print("\n\n", .{});
        }
        grid_iter.reset();

        switch (i) {
            0 => try std.testing.expectEqualSlices(Tile, cycle1grid.array.items, grid.array.items),
            1 => try std.testing.expectEqualSlices(Tile, cycle2grid.array.items, grid.array.items),
            2 => try std.testing.expectEqualSlices(Tile, cycle3grid.array.items, grid.array.items),
            else => {},
        }
    }
}
