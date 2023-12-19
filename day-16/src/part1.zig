const std = @import("std");

const Direction = enum {
    Right,
    Down,
    Left,
    Up,
};

const Tile = enum {
    Empty,
    UpMirror,
    DownMirror,
    VertialSplit,
    HorizontalSplit,
};

const EnergizedMap = std.AutoHashMap(usize, [4]bool);
const Layout = struct {
    grid: std.ArrayList(Tile),
    width: usize,
    height: usize,

    fn from_string(allocator: std.mem.Allocator, input: []const u8) !Layout {
        var layout: Layout = undefined;
        layout.grid = std.ArrayList(Tile).init(allocator);

        layout.height = 1;
        layout.width = 0;
        for (input) |c| {
            layout.width += 1;
            const tile: Tile = switch (c) {
                '.' => .Empty,
                '/' => .UpMirror,
                '\\' => .DownMirror,
                '|' => .VertialSplit,
                '-' => .HorizontalSplit,
                '\n' => {
                    layout.height += 1;
                    layout.width = 0;
                    continue;
                },
                else => continue,
            };
            try layout.grid.append(tile);
        }
        std.debug.print("Grid: {d} rows, {d} cols\n", .{ layout.height, layout.width });
        return layout;
    }

    fn deinit(self: Layout) void {
        self.grid.deinit();
    }

    fn walk(self: Layout, pos: usize, direction: Direction, energized_tiles: *EnergizedMap) !void {
        const entry = try energized_tiles.getOrPut(pos);
        if (!entry.found_existing) {
            entry.value_ptr.* = [4]bool{ false, false, false, false };
            entry.value_ptr.*[@intFromEnum(direction)] = true;
        } else if (entry.value_ptr.*[@intFromEnum(direction)]) {
            std.debug.print("Direction repeat on {d}\n", .{pos});
            return;
        }

        const tile = self.grid.items[pos];
        std.debug.print("Pos: {d}, tile: {}\n", .{ pos, tile });
        for (switch (tile) {
            .Empty => [_]?Direction{ direction, null },
            .HorizontalSplit => switch (direction) {
                .Right, .Left => [_]?Direction{ direction, null },
                .Down, .Up => [_]?Direction{ .Right, .Left },
            },
            .VertialSplit => switch (direction) {
                .Down, .Up => [_]?Direction{ direction, null },
                .Right, .Left => [_]?Direction{ .Down, .Up },
            },
            .UpMirror => [_]?Direction{ switch (direction) {
                .Right => .Up,
                .Down => .Left,
                .Left => .Down,
                .Up => .Right,
            }, null },
            .DownMirror => [_]?Direction{ switch (direction) {
                .Right => .Down,
                .Down => .Right,
                .Left => .Up,
                .Up => .Left,
            }, null },
        }) |new_direction| {
            if (new_direction == null) continue;
            const new_pos = self.next_pos(pos, new_direction.?) orelse {
                std.debug.print("Next_pos failed at pos {d}\n", .{pos});
                return;
            };
            try self.walk(new_pos, new_direction.?, energized_tiles);
        }
    }

    fn next_pos(self: Layout, pos: usize, direction: Direction) ?usize {
        switch (direction) {
            .Right => {
                const column = pos % self.width;
                if (column >= self.width - 1) {
                    return null;
                }
                return pos + 1;
            },
            .Down => {
                const new_pos = pos + self.width;
                if (new_pos >= self.grid.items.len) {
                    return null;
                }
                return new_pos;
            },
            .Left => {
                const column = pos % self.width;
                if (column == 0) {
                    return null;
                }
                return pos - 1;
            },
            .Up => {
                if (pos < self.width) {
                    return null;
                }
                return pos - self.width;
            },
        }
    }

    fn print(self: Layout) void {
        for (self.grid.items, 0..) |tile, i| {
            if (i % self.width == 0) {
                std.debug.print("\n", .{});
            }
            const tile_char: u8 = switch (tile) {
                .Empty => '.',
                .HorizontalSplit => '-',
                .VertialSplit => '|',
                .DownMirror => '\\',
                .UpMirror => '/',
            };
            std.debug.print("{c}", .{tile_char});
        }
    }

    fn print_energized(self: Layout, energized_tiles: *const EnergizedMap) void {
        for (self.grid.items, 0..) |tile, i| {
            _ = tile;
            if (i % self.width == 0) {
                std.debug.print("\n", .{});
            }
            if (energized_tiles.contains(i)) {
                std.debug.print("#", .{});
            } else {
                std.debug.print(".", .{});
            }
        }
    }
};

fn process(allocator: std.mem.Allocator, input: []const u8) !usize {
    var layout = try Layout.from_string(allocator, input);
    defer layout.deinit();

    var energized_tiles = EnergizedMap.init(allocator);
    defer energized_tiles.deinit();

    try layout.walk(0, .Right, &energized_tiles);

    return energized_tiles.count();
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    const input = @embedFile("input.txt");

    const out = try process(allocator, input);
    std.debug.print("\nOUT: {d}\n", .{out});
}

test "example" {
    const input =
        \\.|...\....
        \\|.-.\.....
        \\.....|-...
        \\........|.
        \\..........
        \\.........\
        \\..../.\\..
        \\.-.-/..|..
        \\.|....-|.\
        \\..//.|....
    ;
    try std.testing.expectEqual(@as(usize, 46), try process(std.testing.allocator, input));
}
