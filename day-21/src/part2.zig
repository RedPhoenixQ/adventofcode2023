const std = @import("std");

const Tile = enum {
    Garden,
    Rock,
    Start,
};

const Direction = enum {
    Up,
    Down,
    Left,
    Right,
};

const MoveCache = struct {
    is_even_step: bool,
    x: isize,
    y: isize,
};

const Grid = struct {
    slice: []Tile,
    width: usize,
    height: usize,
    allocator: std.mem.Allocator,

    fn deinit(self: Grid) void {
        self.allocator.free(self.slice);
    }

    fn from_string(allocator: std.mem.Allocator, input: []const u8) !Grid {
        var grid: Grid = undefined;

        grid.allocator = allocator;
        grid.width = std.ascii.indexOfIgnoreCase(input, "\n") orelse unreachable;
        grid.height = input.len / grid.width;
        grid.slice = try allocator.alloc(Tile, grid.width * grid.height);

        var skipped: usize = 0;
        for (input, 0..) |c, i| {
            grid.slice[i - skipped] = switch (c) {
                '.' => .Garden,
                '#' => .Rock,
                'S' => .Start,
                else => {
                    skipped += 1;
                    continue;
                },
            };
        }

        return grid;
    }

    fn get(self: Grid, in_x: isize, in_y: isize) ?Tile {
        const x: usize = if (in_x < 0)
            self.width - (std.math.absCast(in_x + 1) % self.width) - 1
        else
            @as(usize, @intCast(in_x)) % self.width;
        const y: usize = if (in_y < 0)
            self.height - ((std.math.absCast(in_y + 1) % self.height)) - 1
        else
            @as(usize, @intCast(in_y)) % self.height;
        //std.debug.print("{d}, {d}: {d} {d}: {d}\n", .{ in_x, in_y, x, y, x + y * self.width });
        return self.slice[x + y * self.width];
    }

    fn walk(self: Grid, x: isize, y: isize, remaining_steps: usize, visited: *std.AutoHashMap(MoveCache, usize)) !void {
        //std.debug.print("Step: {d}, ({d}, {d})\n", .{ remaining_steps, x, y });
        var next_x: isize = x;
        var next_y: isize = y;
        for (&[_]Direction{ .Up, .Down, .Left, .Right }) |direction| {
            switch (direction) {
                .Up => {
                    next_x = x;
                    next_y = y - 1;
                },
                .Down => {
                    next_x = x;
                    next_y = y + 1;
                },
                .Left => {
                    next_x = x - 1;
                    next_y = y;
                },
                .Right => {
                    next_x = x + 1;
                    next_y = y;
                },
            }

            const key = MoveCache{ .x = next_x, .y = next_y, .is_even_step = remaining_steps % 2 == 0 };
            const cached = visited.get(key);

            const tile = self.get(next_x, next_y);
            if (remaining_steps > 0 and tile != null and tile != .Rock and (cached == null or cached.? < remaining_steps)) {
                try visited.put(key, remaining_steps);
                try self.walk(next_x, next_y, remaining_steps - 1, visited);
            }
        }
    }
};

fn process(allocator: std.mem.Allocator, input: []const u8, steps: usize) !usize {
    const grid = try Grid.from_string(allocator, input);
    defer grid.deinit();

    var start_x: isize = undefined;
    var start_y: isize = undefined;
    for (0..grid.height) |y| {
        for (0..grid.width) |x| {
            if (grid.get(@intCast(x), @intCast(y)) == .Start) {
                start_x = @intCast(x);
                start_y = @intCast(y);
                break;
            }
        }
    }

    var visited = std.AutoHashMap(MoveCache, usize).init(allocator);
    defer visited.deinit();

    try grid.walk(start_x, start_y, steps, &visited);

    const should_be_even = steps % 2 != 0;
    var end_positions: usize = 0;

    std.debug.print("Total moves {d}\n", .{visited.count()});
    var moves = visited.keyIterator();
    while (moves.next()) |move| {
        //std.debug.print("{any}\n", .{move});
        if (move.*.is_even_step == should_be_even) {
            end_positions += 1;
        }
    }

    return end_positions;
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    const input = @embedFile("input.txt");

    const out = try process(allocator, input, 26501365);
    std.debug.print("OUT: {d}\n", .{out});
}

test "example" {
    const input =
        \\...........
        \\.....###.#.
        \\.###.##..#.
        \\..#.#...#..
        \\....#.#....
        \\.##..S####.
        \\.##..#...#.
        \\.......##..
        \\.##.#.####.
        \\.##..##.##.
        \\...........
    ;

    try std.testing.expectEqual(@as(usize, 16), try process(std.testing.allocator, input, 6));
    try std.testing.expectEqual(@as(usize, 50), try process(std.testing.allocator, input, 10));
    try std.testing.expectEqual(@as(usize, 1594), try process(std.testing.allocator, input, 50));
    try std.testing.expectEqual(@as(usize, 6536), try process(std.testing.allocator, input, 100));
    try std.testing.expectEqual(@as(usize, 167004), try process(std.testing.allocator, input, 500));
    try std.testing.expectEqual(@as(usize, 668697), try process(std.testing.allocator, input, 1000));
    try std.testing.expectEqual(@as(usize, 16733044), try process(std.testing.allocator, input, 5000));
}

test "grid indicies" {
    const input =
        \\...........
        \\.....###.#.
        \\.###.##..#.
        \\..#.#...#..
        \\....#.#....
        \\.##..S####.
        \\.##..#...#.
        \\.......##..
        \\.##.#.####.
        \\.##..##.##.
        \\...........
    ;

    const grid = try Grid.from_string(std.testing.allocator, input);
    defer grid.deinit();

    try std.testing.expectEqual(@as(?Tile, .Start), grid.get(5, 5));
    try std.testing.expectEqual(@as(?Tile, .Start), grid.get(16, 5));
    try std.testing.expectEqual(@as(?Tile, .Start), grid.get(-6, 5));
    try std.testing.expectEqual(@as(?Tile, .Start), grid.get(-6, -6));
    try std.testing.expectEqual(@as(?Tile, .Start), grid.get(-17, -6));
}
