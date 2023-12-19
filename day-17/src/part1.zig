const std = @import("std");

const Direction = enum { Up, Down, Left, Right };

const Item = struct {};

fn leastHeatloss(ctx: void, a: Item, b: Item) std.math.Order {

}

fn process(allocator: std.mem.Allocator, input: []const u8) !usize {
    _ = allocator;
    const width = std.ascii.indexOfIgnoreCase(input, "\n") orelse unreachable;
    const height = @as(usize, input.len / width);
    std.debug.print("Width {d}, height {d}", .{ width, height });

    const queue = std.PriorityQueue(Item, void, leastHeatloss).init(allocator, {});

    var min_heap = MinHeap(comptime T: type)

    return 0;
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    const input = @embedFile("input.txt");

    const out = try process(allocator, input);
    std.debug.print("OUT: {d}", .{out});
}

test "example" {
    const input =
        \\2413432311323
        \\3215453535623
        \\3255245654254
        \\3446585845452
        \\4546657867536
        \\1438598798454
        \\4457876987766
        \\3637877979653
        \\4654967986887
        \\4564679986453
        \\1224686865563
        \\2546548887735
        \\4322674655533
    ;
    try std.testing.expectEqual(@as(usize, 102), try process(std.testing.allocator, input));
}